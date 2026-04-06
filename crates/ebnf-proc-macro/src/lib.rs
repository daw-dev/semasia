use ebnf_parser::EbnfProduction;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro]
pub fn ebnf(input: TokenStream) -> TokenStream {
    let ebnf_production: EbnfProduction =
        syn::parse(input).expect("ebnf production not recognized");

    let (productions, types) = ebnf_production.compile();

    let items = types
        .into_iter()
        .map(|ty| match ty {
            ebnf_parser::EbnfCompiledType::Enum {
                enum_ident,
                enum_variants,
            } => {
                let enum_variants = enum_variants
                    .into_iter()
                    .map(|(ident, body)| quote!(#ident(#(#body),*)));
                quote!(pub enum #enum_ident { #(#enum_variants,)*})
            }
            ebnf_parser::EbnfCompiledType::Repetition {
                alias_ident,
                alias_vec_types,
            } => quote!(pub type #alias_ident = Vec<(#(#alias_vec_types),*)>;),
            ebnf_parser::EbnfCompiledType::Optional {
                alias_ident,
                alias_opt_types,
            } => quote!(pub type #alias_ident = Option<(#(#alias_opt_types),*)>;),
        })
        .chain(productions.into_iter().map(|production| {
            let ident = &production.ident;
            let head = &production.head;
            let body = production.body.as_slice();
            let sem_action = match production.sem_action {
                ebnf_parser::CompiledSemAction::Alternative => {
                    let [enum_variant] = body else {
                        panic!("alternative with other than only one body item")
                    };
                    Some(quote!(|t| #head::#enum_variant(t)))
                }
                // not so easy: if body is more than one element you have to put t1, t2, t3, ...
                ebnf_parser::CompiledSemAction::RepetitionSepMore(len) => {
                    let vars = (0..len).map(|i| format_ident!("t{i}")).collect::<Vec<_>>();
                    let fillers = std::iter::repeat_n(format_ident!("_"), body.len() - len - 1);
                    Some(quote!(|(mut acc, #(#fillers,)* #(#vars),*)| {
                        acc.push((#(#vars),*));
                        acc
                    }))
                }
                ebnf_parser::CompiledSemAction::RepetitionSepDone => Some(quote!(|t| vec![t])),
                ebnf_parser::CompiledSemAction::RepetitionEmpty => {
                    Some(quote!(|_| Vec::with_capacity(0)))
                }
                ebnf_parser::CompiledSemAction::RepetitionNonEmpty => None,
                ebnf_parser::CompiledSemAction::SimpleRepetitionMore => {
                    let vars = (0..production.body.len() - 1)
                        .map(|i| format_ident!("t{i}"))
                        .collect::<Vec<_>>();
                    Some(quote!(|(mut acc, #(#vars),*)| {
                        acc.push((#(#vars),*));
                        acc
                    }))
                }
                ebnf_parser::CompiledSemAction::SimpleRepetitionDone => {
                    Some(quote!(|_| Vec::new()))
                }
                ebnf_parser::CompiledSemAction::OptionalSome => Some(quote!(|t| Some(t))),
                ebnf_parser::CompiledSemAction::OptionalNone => Some(quote!(|_| None)),
                ebnf_parser::CompiledSemAction::Compiled(sem_action) => {
                    sem_action.map(|act| quote!(#act))
                }
            };

            let body = match body {
                [single_elem] => quote!(#single_elem),
                body => quote!((#(#body),*)),
            };

            let sem_action = sem_action.map(|act| quote!(, #act));

            quote!(production!(#ident: #head -> #body #sem_action);)
        }));

    quote! {
        #(#items)*
    }
    .into()
}
