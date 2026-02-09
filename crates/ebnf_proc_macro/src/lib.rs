use std::collections::HashSet;

use ebnf_parser::EbnfProduction;
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[derive(Hash, PartialEq, Eq)]
enum ExtraNonTerminalType {
    Repetition(Ident),
    Optional(Ident),
}

#[derive(Hash, PartialEq, Eq)]
struct ExtraNonTerminal {
    ident: Ident,
    ty: ExtraNonTerminalType,
}

#[proc_macro]
pub fn ebnf(input: TokenStream) -> TokenStream {
    todo!()
    // let ebnf_production: EbnfProduction =
    //     syn::parse(input).expect("ebnf production not recognized");
    // let mut items = Vec::new();
    //
    // let ebnf_production_name = ebnf_production.ident;
    // let head = ebnf_production.head;
    // let sem_action = ebnf_production.sem_action;
    // let mut extra_nts = HashSet::new();
    // let mut computed_body = Vec::new();
    //
    // for (item_index, body_item) in ebnf_production.body.into_iter().enumerate() {
    //     match body_item {
    //         ebnf_parser::EbnfBodyItem::Alternatives {
    //             enum_ident,
    //             variants_types,
    //         } => {
    //             items.push(quote! {
    //                 enum #enum_ident {
    //                     #(#variants_types(#variants_types),)*
    //                 }
    //             });
    //             for variant in variants_types.into_iter() {
    //                 let prod_name = EbnfProduction::alternative_ident(
    //                     &ebnf_production_name,
    //                     item_index,
    //                     &variant,
    //                 );
    //                 items.push(quote! {
    //                     production!(#prod_name, #enum_ident -> #variant, |t| #enum_ident::#variant(t));
    //                 })
    //             }
    //             computed_body.push(enum_ident);
    //         }
    //         ebnf_parser::EbnfBodyItem::Repetition(ident) => {
    //             let rep_ident = EbnfProduction::repetition_alias(&ebnf_production_name, item_index);
    //             extra_nts.insert(ExtraNonTerminal {
    //                 ident: ident.clone(),
    //                 ty: ExtraNonTerminalType::Repetition(rep_ident.clone()),
    //             });
    //
    //             let prod_name_more =
    //                 EbnfProduction::repetition_more_ident(&ebnf_production_name, item_index);
    //             let prod_name_done =
    //                 EbnfProduction::repetition_done_ident(&ebnf_production_name, item_index);
    //
    //             items.push(quote! {
    //                 type #rep_ident = Vec<#ident>;
    //
    //                 production!(#prod_name_more, #rep_ident -> (#rep_ident, #ident), |(mut acc, t)| {
    //                     acc.push(t);
    //                     acc
    //                 });
    //
    //                 production!(#prod_name_done, #rep_ident -> (), |_| Vec::new());
    //             });
    //
    //             computed_body.push(rep_ident);
    //         }
    //         ebnf_parser::EbnfBodyItem::Optional(ident) => {
    //             let opt_ident = EbnfProduction::optional_alias(&ebnf_production_name, item_index);
    //             extra_nts.insert(ExtraNonTerminal {
    //                 ident: ident.clone(),
    //                 ty: ExtraNonTerminalType::Optional(opt_ident.clone()),
    //             });
    //
    //             let prod_name_some =
    //                 EbnfProduction::optional_some_ident(&ebnf_production_name, item_index);
    //             let prod_name_none =
    //                 EbnfProduction::optional_none_ident(&ebnf_production_name, item_index);
    //
    //             items.push(quote! {
    //                 type #opt_ident = Option<#ident>;
    //
    //                 production!(#prod_name_some, #opt_ident -> #ident, |t| Some(t));
    //
    //                 production!(#prod_name_none, #opt_ident -> (), |_| None);
    //             });
    //
    //             computed_body.push(opt_ident);
    //         }
    //         ebnf_parser::EbnfBodyItem::Ident(ident) => {
    //             computed_body.push(ident);
    //         }
    //     }
    // }
    //
    // let ebnf_root_production = match sem_action {
    //     Some(sem_action) => quote! {
    //         production!(#ebnf_production_name, #head -> (#(#computed_body),*), #sem_action);
    //     },
    //     None => quote! {
    //         production!(#ebnf_production_name, #head -> (#(#computed_body),*));
    //     },
    // };
    //
    // items.push(ebnf_root_production);
    //
    // quote! {
    //     #(#items)*
    // }
    // .into()
}
