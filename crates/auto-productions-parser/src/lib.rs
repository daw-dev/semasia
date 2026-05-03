use dyn_grammar::{EnrichedBaseProduction, grammar::Body};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, parse::Parse, spanned::Spanned};

#[derive(Debug)]
pub struct AutoProductionsEnumVariant {
    pub ident: Ident,
    pub ty: Vec<(Type, bool)>,
}

impl TryFrom<&mut syn::Variant> for AutoProductionsEnumVariant {
    type Error = syn::Error;

    fn try_from(value: &mut syn::Variant) -> Result<Self, Self::Error> {
        let ident = value.ident.clone();

        let syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) =
            std::mem::replace(&mut value.fields, syn::Fields::Unit)
        else {
            return Err(syn::Error::new(
                value.fields.span(),
                "auto production works only for unnamed fields",
            ));
        };

        let mut fields = syn::punctuated::Punctuated::new();

        let ty = unnamed
            .into_iter()
            .map(|field| {
                let hide = field.attrs.iter().any(|attr| attr.path().is_ident("hide"));
                if !hide {
                    fields.push(field.clone());
                }
                (field.ty, hide)
            })
            .collect_vec();

        value.fields = if fields.is_empty() {
            syn::Fields::Unit
        } else {
            syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: Default::default(),
                unnamed: fields,
            })
        };

        Ok(Self { ident, ty })
    }
}

impl TryFrom<&syn::Variant> for AutoProductionsEnumVariant {
    type Error = syn::Error;

    fn try_from(value: &syn::Variant) -> Result<Self, Self::Error> {
        let ident = value.ident.clone();

        let syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) = &value.fields else {
            return Err(syn::Error::new(
                value.fields.span(),
                "auto production works only for unnamed fields",
            ));
        };

        let ty = unnamed
            .into_iter()
            .map(|field| {
                (
                    field.ty.clone(),
                    field.attrs.iter().any(|attr| attr.path().is_ident("hide")),
                )
            })
            .collect_vec();

        Ok(Self { ident, ty })
    }
}

impl AutoProductionsEnumVariant {
    fn ident_from_type((ty, hide): (Type, bool)) -> (Ident, bool) {
        match ty {
            Type::Path(type_path) => {
                let last_segment = type_path.path.segments.into_iter().last().unwrap();
                match last_segment.arguments {
                    syn::PathArguments::None => (last_segment.ident, hide),
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) if args.len() == 1 => {
                        let inner = args.into_iter().next().unwrap();
                        match inner {
                            syn::GenericArgument::Type(ty) => Self::ident_from_type((ty, hide)),
                            _ => panic!("generic argument must be a type"),
                        }
                    }
                    _ => panic!("arguments must be angled arguments"),
                }
            }
            Type::Paren(_) => panic!("remove the parenthesis"),
            Type::Array(_) => panic!("array not implemented yet"),
            Type::BareFn(_) => panic!("fn is not a valid type"),
            Type::Group(_) => panic!("what even is a group type"),
            Type::Macro(_) => panic!("macro not a valid type"),
            Type::Never(_) => panic!("never not a valid type"),
            Type::Ptr(_) => panic!("pointer not a valid type"),
            Type::Reference(_) => panic!("reference not a valid type"),
            Type::Slice(_) => panic!("slice not a valid type"),
            Type::TraitObject(_) => panic!("trait object not a valid type"),
            Type::Tuple(_) => panic!("tuple not a valid type"),
            _ => unreachable!(),
        }
    }

    fn production_ident(&self, enum_ident: &Ident) -> Ident {
        Ident::new(
            &format!("__Auto{}Is{}", enum_ident, self.ident),
            enum_ident.span(),
        )
    }

    fn compile_type(ty: Vec<(Type, bool)>) -> Vec<(Ident, bool)> {
        ty.into_iter().map(Self::ident_from_type).collect()
    }

    pub fn compile(self, enum_ident: &Ident) -> EnrichedBaseProduction {
        EnrichedBaseProduction::new(
            self.production_ident(enum_ident),
            enum_ident.clone(),
            Body::new(
                Self::compile_type(self.ty)
                    .into_iter()
                    .map(|(ty, _)| ty)
                    .collect(),
            ),
            None,
        )
    }
}

#[derive(Debug)]
pub struct AutoProductionsEnum {
    pub ident: Ident,
    pub variants: Vec<AutoProductionsEnumVariant>,
}

impl TryFrom<&mut syn::Item> for AutoProductionsEnum {
    type Error = syn::Error;

    fn try_from(value: &mut syn::Item) -> Result<Self, Self::Error> {
        let syn::Item::Enum(enum_input) = value else {
            return Err(syn::Error::new(
                value.span(),
                "auto production only works with enums",
            ));
        };

        Ok(AutoProductionsEnum {
            ident: enum_input.ident.clone(),
            variants: enum_input
                .variants
                .iter_mut()
                .map(TryFrom::try_from)
                .process_results(|vars| vars.collect())?,
        })
    }
}

impl TryFrom<&syn::Item> for AutoProductionsEnum {
    type Error = syn::Error;

    fn try_from(value: &syn::Item) -> Result<Self, Self::Error> {
        let syn::Item::Enum(enum_input) = value else {
            return Err(syn::Error::new(
                value.span(),
                "auto production only works with enums",
            ));
        };

        Ok(AutoProductionsEnum {
            ident: enum_input.ident.clone(),
            variants: enum_input
                .variants
                .iter()
                .map(TryFrom::try_from)
                .process_results(|vars| vars.collect())?,
        })
    }
}

impl Parse for AutoProductionsEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        AutoProductionsEnum::try_from(&mut input.parse::<syn::Item>()?)
    }
}

impl ToTokens for AutoProductionsEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        for variant in self.variants.iter() {
            let variant_ident = &variant.ident;
            let variant_ty = &variant.ty;
            let prod_ident = variant.production_ident(ident);
            let compiled_type = AutoProductionsEnumVariant::compile_type(variant_ty.clone());
            let prod_body = compiled_type.iter().map(|(ty, _)| ty);
            let mut temp_counter = 0;
            let patt = compiled_type.iter().map(|(_, hide)| {
                if *hide {
                    format_ident!("_")
                } else {
                    let res = format_ident!("t{temp_counter}");
                    temp_counter += 1;
                    res
                }
            });
            let temps = compiled_type
                .iter()
                .filter_map(|(_, hide)| (!hide).then_some(()))
                .enumerate()
                .map(|(i, _)| format_ident!("t{i}"))
                .collect_vec();

            let production = if temps.is_empty() {
                quote!(
                    production!(#prod_ident: #ident -> (#(#prod_body),*), |(#(#patt),*)| #ident::#variant_ident);
                )
            } else {
                quote!(
                    production!(#prod_ident: #ident -> (#(#prod_body),*), |(#(#patt),*)| #ident::#variant_ident(#(#temps.into()),*));
                )
            };
            tokens.extend(production);
        }
    }
}

impl AutoProductionsEnum {
    pub fn compile(self) -> Vec<EnrichedBaseProduction> {
        self.variants
            .into_iter()
            .map(|var| var.compile(&self.ident))
            .collect()
    }
}
