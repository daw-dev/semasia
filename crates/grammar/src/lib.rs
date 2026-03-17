use crate::constructor::Constructor;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error, set_dummy};
use quote::quote;
use syn::{File, Ident, ItemMod};

mod constructor;
mod grammar_extraction;
mod item_injections;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn grammar(attr: TokenStream, item: TokenStream) -> TokenStream {
    let internal_mod_name = syn::parse::<Ident>(attr).ok();
    if let Ok(mut module) = syn::parse::<ItemMod>(item.clone()) {
        let ident = &module.ident;
        set_dummy(quote! {
            mod #ident {
                pub type Parser = parser::dummy::DummyParser;
            }
        });
        let (_, items) = module
            .content
            .as_mut()
            .expect("grammar module must be inline (contain braces)");

        let extracted = Constructor.extract(items);
        let simplified = extracted.simplify();
        let analyzed = simplified.analyze();
        analyzed.inject_items(items, internal_mod_name);

        quote! { #module }.into()
    } else if let Ok(File { items, .. }) = &mut syn::parse(item) {
        let extracted = Constructor.extract(items);
        let simplified = extracted.simplify();
        let analyzed = simplified.analyze();
        analyzed.inject_items(items, internal_mod_name);

        quote! { #(#items)* }.into()
    } else {
        abort_call_site!("a grammar is either an inline module or a file");
    }
}

macro_rules! dummy_attribute {
    ($attr:ident, $pos:expr) => {
        #[proc_macro_error]
        #[proc_macro_attribute]
        pub fn $attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
            set_dummy(item.clone().into());
            abort_call_site!("this attribute has to be put on top of {}", $pos)
        }
    };
}

dummy_attribute!(token, "type aliases, structs, enums or use directives");
dummy_attribute!(
    start_symbol,
    "type aliases, structs, enums or use directives"
);
dummy_attribute!(
    non_terminal,
    "type aliases, structs, enums or use directives"
);
dummy_attribute!(left_associative, "tokens");
dummy_attribute!(right_associative, "tokens");
dummy_attribute!(priority, "tokens or productions");
dummy_attribute!(
    context,
    "ONLY ONE type alias, struct, enum or use directive"
);
