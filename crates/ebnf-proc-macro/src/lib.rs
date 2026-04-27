use ebnf_parser::EbnfProduction;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn ebnf(input: TokenStream) -> TokenStream {
    let ebnf_production: EbnfProduction =
        syn::parse(input).expect("ebnf production not recognized");

    let (productions, types) = ebnf_production.compile();

    let res = quote! {
        #(#types)*
        #(production!(#productions);)*
    }
    .into();

    // eprintln!("{res}");

    res
}
