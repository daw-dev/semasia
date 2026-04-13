use auto_productions_parser::AutoProductionsEnum;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn auto_productions(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let auto_productions_enum: AutoProductionsEnum =
        syn::parse(item.clone()).expect("couldn't parse the item");

    let item: proc_macro2::TokenStream = item.into();

    let res = quote! {
        #item
        #auto_productions_enum
    }
    .into();

    eprintln!("{res}");

    res
}
