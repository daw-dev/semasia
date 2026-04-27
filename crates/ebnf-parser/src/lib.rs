use dyn_grammar::grammar::{Body, Production};
use itertools::Itertools;
use proc_macro2::Span;
use quote::format_ident;
use syn::{
    Attribute, ExprClosure, Ident, ItemType, LitInt, Token, bracketed, parenthesized, parse::Parse,
    parse_quote, spanned::Spanned,
};

pub type EbnfCompiledType = ItemType;
pub type CompiledSemAction = ExprClosure;
pub type EbnfCompiledProduction = Production<Ident, Ident, Ident, Option<ExprClosure>>;

type Separator = EbnfBody;

#[derive(Debug)]
enum EbnfBodyItem {
    Vec {
        separator: Option<Separator>,
        body: EbnfBody,
    },
    Array {
        separator: Option<Separator>,
        body: EbnfBody,
        size: usize,
    },
    Option {
        body: EbnfBody,
    },
    Ident {
        ident: Ident,
    },
}

impl Parse for EbnfBodyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr = input.call(Attribute::parse_outer).ok();
        let attr = match attr {
            None => None,
            Some(empty) if empty.is_empty() => None,
            Some(mut one) if one.len() == 1 => Some(one.pop().unwrap()),
            Some(multiple) => {
                return Err(syn::Error::new(
                    multiple[1].span(),
                    "only one attribute is accepted on ebnf body items",
                ));
            }
        };

        if let Ok(ident) = input.parse::<Ident>() {
            if input.parse::<Token![<]>().is_ok() {
                let body = input.parse::<EbnfBody>()?;
                input.parse::<Token![>]>()?;
                if ident == "Vec" {
                    let separator = match attr {
                        Some(attr) => {
                            if attr.path().is_ident("separator") {
                                match attr.meta {
                                    syn::Meta::List(meta_list) => {
                                        let body = syn::parse2(meta_list.tokens)?;
                                        Some(body)
                                    }
                                    _ => {
                                        return Err(syn::Error::new(
                                            attr.meta.span(),
                                            "attribute has to have a meta list",
                                        ));
                                    }
                                }
                            } else {
                                return Err(syn::Error::new(
                                    attr.path().span(),
                                    "#[separator(Sep)] is the only valid attribute",
                                ));
                            }
                        }
                        None => None,
                    };
                    Ok(Self::Vec { separator, body })
                } else if ident == "Option" {
                    Ok(Self::Option { body })
                } else {
                    Err(syn::Error::new(
                        input.span(),
                        "ebnf body item has to be either an ident, an Option, a Vec or an array",
                    ))
                }
            } else {
                if let Some(attr) = attr {
                    return Err(syn::Error::new(
                        attr.span(),
                        "attribute are valid only on a Vec or array",
                    ));
                }
                Ok(EbnfBodyItem::Ident { ident })
            }
        } else if input.peek(syn::token::Bracket) {
            let inner;
            bracketed!(inner in input);
            let body = inner.parse()?;
            inner.parse::<Token![;]>()?;
            let size = inner.parse::<LitInt>()?;
            let size = size.base10_parse::<usize>()?;
            Ok(Self::Array {
                separator: None,
                body,
                size,
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "ebnf body item has to be either an ident, an Option, a Vec or an array",
            ))
        }
    }
}

impl EbnfBodyItem {
    fn compile_helper(
        self,
        productions: &mut Vec<EbnfCompiledProduction>,
        types: &mut Vec<EbnfCompiledType>,
        id_stack: &mut Vec<String>,
        span: Span,
    ) -> Ident {
        match self {
            EbnfBodyItem::Vec { separator, body } => {
                let alias = Self::repetition_alias(id_stack, span);

                let body_len = body.elems.len();

                let compiled_body = body.compile_helper(productions, types, id_stack, span);

                types.push(parse_quote!(pub type #alias = Vec<(#(#compiled_body),*)>;));

                let compiled_body_len = compiled_body.len();

                match separator {
                    Some(separator) => {
                        let non_empty_rep_alias = Self::non_empty_repetition_alias(id_stack, span);

                        types.push(parse_quote!(pub type #non_empty_rep_alias = #alias;));

                        let separator_body =
                            separator.compile_helper(productions, types, id_stack, span);

                        productions.push(Production::new(
                            Self::repetition_non_empty_ident(id_stack, span),
                            alias.clone(),
                            Body::new(vec![non_empty_rep_alias.clone()]),
                            None,
                        ));

                        productions.push(Production::new(
                            Self::repetition_empty_ident(id_stack, span),
                            alias.clone(),
                            Body::new(Vec::with_capacity(0)),
                            Some(parse_quote!(|_| Vec::new())),
                        ));

                        productions.push(Production::new(
                            Self::repetition_more_ident(id_stack, span),
                            non_empty_rep_alias.clone(),
                            std::iter::once(non_empty_rep_alias.clone())
                                .chain(separator_body.into_iter())
                                .chain(compiled_body.clone())
                                .collect(),
                            Some({
                                let vars = (0..compiled_body_len)
                                    .map(|i| format_ident!("t{i}"))
                                    .collect::<Vec<_>>();
                                let fillers = std::iter::repeat_n(
                                    format_ident!("_"),
                                    body_len - compiled_body_len + 1,
                                );
                                parse_quote!(|(mut acc, #(#fillers,)* #(#vars),*)| {
                                    acc.push((#(#vars),*));
                                    acc
                                })
                            }),
                        ));

                        productions.push(Production::new(
                            Self::repetition_done_ident(id_stack, span),
                            non_empty_rep_alias.clone(),
                            Body::new(compiled_body),
                            Some(parse_quote!(|t| vec![t])),
                        ));
                    }
                    None => {
                        productions.push(Production::new(
                            Self::repetition_more_ident(id_stack, span),
                            alias.clone(),
                            std::iter::once(alias.clone())
                                .chain(compiled_body)
                                .collect(),
                            Some({
                                let vars = (0..compiled_body_len)
                                    .map(|i| format_ident!("t{i}"))
                                    .collect::<Vec<_>>();

                                parse_quote!(|(mut acc, #(#vars),*)| {
                                    acc.push((#(#vars),*));
                                    acc
                                })
                            }),
                        ));

                        productions.push(Production::new(
                            Self::repetition_done_ident(id_stack, span),
                            alias.clone(),
                            Body::new(Vec::with_capacity(0)),
                            Some(parse_quote!(|_| Vec::new())),
                        ));
                    }
                }

                alias
            }
            EbnfBodyItem::Array {
                separator,
                body,
                size,
            } => todo!(),
            EbnfBodyItem::Option { body } => {
                let alias = Self::optional_alias(id_stack, span);

                let compiled_body = body.compile_helper(productions, types, id_stack, span);

                types.push(parse_quote!(pub type #alias = Option<(#(#compiled_body),*)>;));

                productions.push(Production::new(
                    Self::optional_some_ident(id_stack, span),
                    alias.clone(),
                    Body::new(compiled_body),
                    Some(parse_quote!(|t| Some(t))),
                ));

                productions.push(Production::new(
                    Self::optional_none_ident(id_stack, span),
                    alias.clone(),
                    Body::new(Vec::with_capacity(0)),
                    Some(parse_quote!(|_| None)),
                ));

                alias
            }
            EbnfBodyItem::Ident { ident } => ident,
        }
    }

    fn compose_name(id_stack: &[String], span: Span) -> Ident {
        Ident::new(&id_stack.iter().format("At").to_string(), span)
    }

    fn repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Rep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn non_empty_repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("NonEmptyRep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_more_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("More".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_done_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Done".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_empty_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Empty".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_non_empty_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("NonEmpty".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Opt".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_some_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Some".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_none_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("None".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }
}

#[derive(Debug)]
struct EbnfBody {
    elems: Vec<EbnfBodyItem>,
}

impl Parse for EbnfBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elems = if input.peek(syn::token::Paren) {
            let body_content;
            parenthesized!(body_content in input);
            let body = body_content.parse_terminated(EbnfBodyItem::parse, Token![,])?;
            body.into_iter().collect()
        } else {
            vec![input.parse()?]
        };

        Ok(Self { elems })
    }
}

impl EbnfBody {
    fn compile_helper(
        self,
        productions: &mut Vec<EbnfCompiledProduction>,
        types: &mut Vec<EbnfCompiledType>,
        id_stack: &mut Vec<String>,
        span: Span,
    ) -> Vec<Ident> {
        self.elems
            .into_iter()
            .enumerate()
            .map(|(i, elem)| {
                id_stack.push(format!("{i}"));
                let compiled = elem.compile_helper(productions, types, id_stack, span);
                id_stack.pop();
                compiled
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct EbnfProduction {
    ident: Ident,
    head: Ident,
    body: EbnfBody,
    sem_action: Option<ExprClosure>,
}

impl EbnfProduction {
    pub fn compile(self) -> (Vec<EbnfCompiledProduction>, Vec<EbnfCompiledType>) {
        let ident = self.ident;
        let mut productions = Vec::with_capacity(self.body.elems.len());
        let mut types = Vec::with_capacity(self.body.elems.len());
        let head = self.head;

        let compiled_body = self.body.compile_helper(
            &mut productions,
            &mut types,
            &mut vec!["__Ebnf".to_string(), ident.to_string()],
            ident.span().clone(),
        );

        productions.push(Production::new(
            ident,
            head,
            Body::new(compiled_body),
            self.sem_action,
        ));

        (productions, types)
    }
}

impl Parse for EbnfProduction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let head = input.parse()?;
        input.parse::<Token![->]>()?;
        let body = input.parse()?;

        if input.is_empty() {
            return Ok(EbnfProduction {
                ident,
                head,
                body,
                sem_action: None,
            });
        }

        input.parse::<Token![,]>()?;
        let sem_action = input.parse()?;

        Ok(EbnfProduction {
            ident,
            head,
            body,
            sem_action: Some(sem_action),
        })
    }
}
