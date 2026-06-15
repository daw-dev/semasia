use semasia_auto_productions_parser::AutoProductionsEnum;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn auto_productions(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut syn_item: syn::Item = 
        syn::parse(item).expect("couldn't parse the item");
    let auto_productions_enum: AutoProductionsEnum = (&mut syn_item).try_into().expect("wrong");

    quote! {
        #syn_item
        #auto_productions_enum
    }
    .into()
}
