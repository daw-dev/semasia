use ebnf_parser::EbnfProduction;
use proc_macro::TokenStream;
use quote::quote;

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
                ebnf_parser::CompiledSemAction::RepetitionMore => Some(quote!(|(mut acc, t)| {
                    acc.push(t);
                    acc
                })),
                ebnf_parser::CompiledSemAction::RepetitionDone => Some(quote!(|_| Vec::new())),
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

            quote!(production!(#ident, #head -> #body, #sem_action);)
        }));

    let res = quote! {
        #(#items)*
    }
    .into();

    eprintln!("{res}");

    res
}
