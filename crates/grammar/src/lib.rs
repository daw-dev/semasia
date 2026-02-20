use crate::{constructor::Constructor};
use proc_macro::TokenStream;
use proc_macro_error::{emit_call_site_error, proc_macro_error};
use quote::quote;
use syn::{File, Ident, Item, ItemMod};

mod constructor;
mod grammar_extraction;
mod item_injections;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn grammar(attr: TokenStream, item: TokenStream) -> TokenStream {
    let internal_mod_name = syn::parse::<Ident>(attr).ok();
    if let Ok(mut module) = syn::parse::<ItemMod>(item.clone()) {
        let (_, items) = module
            .content
            .as_mut()
            .expect("grammar module must be inline (contain braces)");

        let constructor = Constructor::extract(items, internal_mod_name);
        constructor.inject_items(items);

        quote! { #module }.into()
    } else if let Ok(File { items, .. }) = &mut syn::parse(item) {
        let constructor = Constructor::extract(items, internal_mod_name);
        constructor.inject_items(items);

        quote! { #(#items)* }.into()
    } else {
        emit_call_site_error!("a grammar is either an inline module or a file");
        panic!()
    }
}

macro_rules! dummy_attribute {
    ($attr:ident, $pos:expr) => {
        #[proc_macro_attribute]
        #[proc_macro_error]
        pub fn $attr(_attr: TokenStream, _item: TokenStream) -> TokenStream {
            panic!("this attribute has to be put on top of {}", $pos)
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
dummy_attribute!(left_associative, "production macros");
dummy_attribute!(right_associative, "production macros");
dummy_attribute!(precedence, "production marcos");

fn extract_ident_from_use_tree(tree: &syn::UseTree) -> Option<&Ident> {
    match tree {
        syn::UseTree::Path(use_path) => extract_ident_from_use_tree(&use_path.tree),
        syn::UseTree::Name(use_name) => Some(&use_name.ident),
        syn::UseTree::Rename(use_rename) => Some(&use_rename.rename),
        syn::UseTree::Group(syn::UseGroup { items, .. }) if items.len() == 1 => {
            extract_ident_from_use_tree(items.first().unwrap())
        }
        _ => None,
    }
}

fn extract_info(item: &Item) -> Option<&Ident> {
    match item {
        Item::Type(syn::ItemType { ident, .. })
        | Item::Struct(syn::ItemStruct { ident, .. })
        | Item::Enum(syn::ItemEnum { ident, .. }) => Some(ident),
        Item::Use(syn::ItemUse { tree, .. }) => extract_ident_from_use_tree(tree),
        _ => None,
    }
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn context(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: Item = syn::parse(item).expect("can't parse context");

    match extract_info(&item) {
        Some(ident) => quote! {
            #item

            type __CompilerContext = #ident;
        }
        .into(),
        None => {
            emit_call_site_error!("context has to be a struct, an enum or a use statement");
            panic!()
        }
    }
}
