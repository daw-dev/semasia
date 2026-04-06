use dyn_grammar::{
    EnrichedBaseProduction, EnrichedGrammar, EnrichedNonTerminal, EnrichedToken,
    conflicts::Associativity, grammar::Body, lalr::LalrAutomaton,
    symbolic_grammar::SymbolicGrammar,
};
use ebnf_parser::EbnfProduction;
use itertools::Itertools;
use proc_macro_error::{abort_if_dirty, emit_call_site_error, emit_call_site_warning, emit_error};
use quote::quote;
use std::collections::HashSet;
use syn::{
    Attribute, Ident, Item, ItemEnum, ItemStruct, ItemType, ItemUse, LitInt, Meta, Type, UseGroup,
    UseTree, parse::Parser, parse_quote,
};

use crate::constructor::*;

impl Constructor {
    pub fn extract(self, items: &mut [Item]) -> Extracted {
        let mut tokens = Vec::new();
        let mut non_terminals = Vec::new();
        let mut ebnf_extra_non_terminals = HashSet::new();
        let mut productions = Vec::new();
        let mut start_symbol = None;
        let mut compiler_ctx: Option<Ident> = None;

        for item in items.iter_mut() {
            if let Some(ctx) = Self::extract_context(item) {
                if let Some(old_ctx) = compiler_ctx.as_ref() {
                    emit_error!(
                        old_ctx, "you can only declare one compilation context";
                        note = ctx.span() => "second compiler context defined here"
                    );
                }
                compiler_ctx = Some(ctx);
            } else if let Some(token) = Self::extract_token(item) {
                tokens.push(token);
            } else if let Some((non_terminal, is_start)) = Self::extract_non_terminal(item) {
                if is_start {
                    if let Some(cur_start) = start_symbol {
                        let start_nt: &EnrichedNonTerminal = &non_terminals[cur_start];
                        emit_error!(
                            start_nt.id(),
                            "you can only declare one start symbol";
                            note = non_terminal.id().span() => "second start symbol defined here"
                        );
                    }
                    start_symbol = Some(non_terminals.len());
                }
                non_terminals.push(non_terminal);
            } else if let Some(production) = Self::extract_production(item) {
                productions.push(production);
            } else if let Some(ebnf) = Self::extract_ebnf_production(item) {
                // eprintln!("found {ebnf}");
                let extra_prods = ebnf.compile().0.into_iter().map(Into::into).collect_vec();
                // for prod in extra_prods.iter() {
                //     eprintln!("{prod}");
                // }
                let extra_nts = extra_prods
                    .iter()
                    .map(EnrichedBaseProduction::head)
                    .map(|head| EnrichedNonTerminal::new(head.clone(), ()));
                ebnf_extra_non_terminals.extend(extra_nts);
                productions.extend(extra_prods);
            }
        }

        if non_terminals.is_empty() || productions.is_empty() {
            emit_call_site_error!(
            "every grammar has to have some non-terminals and productions.";
            note = "Found non-terminals: [{}], tokens: [{}], productions: [{}]",
                non_terminals.iter().format(", "),
                tokens.iter().format(", "),
                productions.iter().format(", ");
            );
        }

        let start_symbol = start_symbol.unwrap_or_else(|| {
            if let Some(nt) = non_terminals.first() {
                emit_call_site_warning!(
                    "no start symbol was declared, using {}", non_terminals[0];
                    help = nt.id().span() => "add #[start_symbol] here"
                );
            }
            0
        });

        non_terminals.extend(ebnf_extra_non_terminals);

        let productions = productions
            .into_iter()
            .map(|prod| prod.into_production(&tokens, &non_terminals))
            .collect();

        let grammar = EnrichedGrammar::new(
            tokens,
            non_terminals.into_iter().unique().collect(),
            start_symbol,
            productions,
            dyn_grammar::Context(compiler_ctx),
        );

        Extracted { grammar }
    }

    fn extract_ident_from_use_tree(tree: &mut UseTree) -> Option<Ident> {
        match tree {
            UseTree::Path(use_path) => Self::extract_ident_from_use_tree(&mut use_path.tree),
            UseTree::Name(use_name) => Some(use_name.ident.clone()),
            UseTree::Rename(use_rename) => Some(use_rename.rename.clone()),
            UseTree::Group(UseGroup { items, .. }) if items.len() == 1 => {
                Self::extract_ident_from_use_tree(items.pop().unwrap().value_mut())
            }
            _ => None,
        }
    }

    fn extract_info(item: &mut Item) -> Option<(&mut Vec<Attribute>, Ident)> {
        match item {
            Item::Type(ItemType { attrs, ident, .. })
            | Item::Struct(ItemStruct { attrs, ident, .. })
            | Item::Enum(ItemEnum { attrs, ident, .. }) => Some((attrs, ident.clone())),
            Item::Use(ItemUse { attrs, tree, .. }) => {
                Self::extract_ident_from_use_tree(tree).map(|ident| (attrs, ident))
            }
            _ => None,
        }
    }

    fn is_marker(item: &Item) -> bool {
        matches!(item, Item::Struct(str) if matches!(str.fields, syn::Fields::Unit))
    }

    fn extract_context(item: &mut Item) -> Option<Ident> {
        let (attrs, ident) = Self::extract_info(item)?;
        let id = attrs.iter().enumerate().find_map(|(i, attr)| {
            if let Meta::Path(path) = &attr.meta
                && path.is_ident("context")
            {
                return Some(i);
            }
            None
        })?;
        attrs.remove(id);
        Some(ident.clone())
    }

    fn extract_token(item: &mut Item) -> Option<EnrichedToken> {
        let is_marker = Self::is_marker(item);
        let (attrs, ident) = Self::extract_info(item)?;
        let token_attrs = attrs
            .extract_if(.., |attr| {
                if !attr.path().is_ident("token") && !attr.path().is_ident("regex") {
                    return false;
                }
                if !is_marker {
                    return true;
                }
                if let Meta::List(raw_list) = &mut attr.meta {
                    let parser =
                        syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
                    if let Ok(mut list) = parser.parse2(raw_list.tokens.clone().into()) {
                        list.insert(1, parse_quote!(|_| #ident));
                        raw_list.tokens = quote!(#list);
                    }
                }
                true
            })
            .collect_vec();
        (!token_attrs.is_empty()).then(|| {
            let mut res_priority = None;
            attrs.retain(|attr| {
                if !attr.path().is_ident("priority") {
                    return true;
                }
                let priority: Result<usize, _> =
                    attr.parse_args_with(|input: syn::parse::ParseStream| {
                        let lit_int: LitInt = input.parse()?;
                        lit_int.base10_parse()
                    });
                if let Ok(priority) = priority {
                    if res_priority.is_some() {
                        emit_error!(attr, "duplicated priority attribute!");
                    }
                    res_priority = Some(priority);
                    return false;
                }
                true
            });
            let mut res_assoc = None;
            attrs.retain(|attr| {
                let assoc = attr
                    .path()
                    .is_ident("left_associative")
                    .then_some(Associativity::Left)
                    .or(attr
                        .path()
                        .is_ident("right_associative")
                        .then_some(Associativity::Right));
                match assoc {
                    Some(assoc) => {
                        if res_assoc.is_some() {
                            emit_error!(attr, "duplicated associativity attribute!");
                        }
                        res_assoc = Some(assoc);
                        false
                    }
                    None => true,
                }
            });
            EnrichedToken::new(
                ident,
                (token_attrs, res_priority, res_assoc.unwrap_or_default()),
            )
        })
    }

    fn extract_non_terminal(item: &mut Item) -> Option<(EnrichedNonTerminal, bool)> {
        let (attrs, ident) = Self::extract_info(item)?;
        let id = attrs.iter().enumerate().find_map(|(i, attr)| {
            if let Meta::Path(path) = &attr.meta
                && path.is_ident("non_terminal")
            {
                return Some(i);
            }
            None
        })?;
        attrs.remove(id);
        let mut is_start = false;
        if let Some(id) = attrs.iter().enumerate().find_map(|(i, attr)| {
            if let Meta::Path(path) = &attr.meta
                && path.is_ident("start_symbol")
            {
                return Some(i);
            }
            None
        }) {
            attrs.remove(id);
            is_start = true;
        }
        Some((EnrichedNonTerminal::new(ident, ()), is_start))
    }

    fn extract_production(item: &mut Item) -> Option<EnrichedBaseProduction> {
        match item {
            Item::Macro(mac) if mac.mac.path.is_ident("production") => {
                let mut res_priority = None;
                mac.attrs.retain(|attr| {
                    if !attr.path().is_ident("priority") {
                        return true;
                    }
                    let priority: Result<usize, _> =
                        attr.parse_args_with(|input: syn::parse::ParseStream| {
                            let lit_int: LitInt = input.parse()?;
                            lit_int.base10_parse()
                        });
                    if let Ok(priority) = priority {
                        if res_priority.is_some() {
                            emit_error!(attr, "duplicated priority attribute!");
                        }
                        res_priority = Some(priority);
                        return false;
                    }
                    true
                });
                mac.mac
                    .parse_body_with(|input: syn::parse::ParseStream| {
                        let name = input.parse()?;
                        input.parse::<syn::Token![:]>()?;
                        let head = input.parse()?;
                        input.parse::<syn::Token![->]>()?;
                        let body = input.parse()?;
                        let body = match body {
                            Type::Path(type_path) => vec![
                                type_path
                                    .path
                                    .get_ident()
                                    .expect("use only one type")
                                    .clone(),
                            ],
                            Type::Tuple(type_tuple) => type_tuple
                                .elems
                                .iter()
                                .map(|t| {
                                    let Type::Path(type_path) = t else {
                                        panic!(
                                            "body of production has to be a tuple of named types"
                                        )
                                    };
                                    type_path
                                        .path
                                        .get_ident()
                                        .expect("tuple of named types")
                                        .clone()
                                })
                                .collect(),
                            _ => panic!("type must be a unit, a single type or a tuple"),
                        };
                        let res = Ok(EnrichedBaseProduction::new(
                            name,
                            head,
                            Body::new(body),
                            res_priority,
                        ));
                        if input.is_empty() {
                            return res;
                        }
                        input.parse::<syn::Token![,]>()?;
                        input.parse::<syn::Expr>()?;
                        res
                    })
                    .ok()
            }
            _ => None,
        }
    }

    fn extract_ebnf_production(item: &mut Item) -> Option<EbnfProduction> {
        match item {
            Item::Macro(mac) if mac.mac.path.is_ident("ebnf") => {
                let res = mac.mac.parse_body::<EbnfProduction>();
                match res {
                    Ok(ebnf) => Some(ebnf),
                    Err(err) => {
                        emit_error!(mac, "ebnf production not well formed"; note = "{}", err);
                        None
                    }
                }
            }
            _ => None,
        }
    }
}

impl Extracted {
    pub fn simplify(self) -> Simplified {
        abort_if_dirty();
        let grammar = SymbolicGrammar::from(self.grammar);
        Simplified { grammar }
    }
}

impl Simplified {
    pub fn analyze(&self) -> Analyzed<'_> {
        let automaton = LalrAutomaton::compute(&self.grammar);
        let (token_table, eof_table, non_terminal_table) = automaton.generate_tables();
        Analyzed {
            automaton,
            token_table,
            eof_table,
            non_terminal_table,
        }
    }
}
