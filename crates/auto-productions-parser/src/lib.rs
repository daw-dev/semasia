use dyn_grammar::{EnrichedBaseProduction, grammar::Body};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, parse::Parse, spanned::Spanned};

pub struct AutoProductionsEnumVariant {
    pub ident: Ident,
    pub ty: Vec<Type>,
}

impl TryFrom<syn::Variant> for AutoProductionsEnumVariant {
    type Error = syn::Error;

    fn try_from(value: syn::Variant) -> Result<Self, Self::Error> {
        let ident = value.ident;

        let syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) = value.fields else {
            return Err(syn::Error::new(
                value.fields.span(),
                "auto production works only for unnamed fields",
            ));
        };

        let ty = unnamed.into_iter().map(|field| field.ty).collect();

        Ok(Self { ident, ty })
    }
}

impl AutoProductionsEnumVariant {
    fn ident_from_type(ty: Type) -> Ident {
        match ty {
            Type::Path(type_path) => {
                let last_segment = type_path.path.segments.into_iter().last().unwrap();
                match last_segment.arguments {
                    syn::PathArguments::None => last_segment.ident,
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) if args.len() == 1 => {
                        let inner = args.into_iter().next().unwrap();
                        match inner {
                            syn::GenericArgument::Type(ty) => Self::ident_from_type(ty),
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

    fn compile_type(ty: Vec<Type>) -> Vec<Ident> {
        ty.into_iter().map(Self::ident_from_type).collect()
    }

    pub fn compile(self, enum_ident: &Ident) -> EnrichedBaseProduction {
        EnrichedBaseProduction::new(
            self.production_ident(enum_ident),
            enum_ident.clone(),
            Body::new(Self::compile_type(self.ty)),
            None,
        )
    }
}

pub struct AutoProductionsEnum {
    pub ident: Ident,
    pub variants: Vec<AutoProductionsEnumVariant>,
}

impl TryFrom<syn::Item> for AutoProductionsEnum {
    type Error = syn::Error;

    fn try_from(value: syn::Item) -> Result<Self, Self::Error> {
        let syn::Item::Enum(enum_input) = value else {
            return Err(syn::Error::new(
                value.span(),
                "auto production only works with enums",
            ));
        };

        Ok(AutoProductionsEnum {
            ident: enum_input.ident,
            variants: enum_input
                .variants
                .into_iter()
                .map(TryFrom::try_from)
                .process_results(|vars| vars.collect())?,
        })
    }
}

impl Parse for AutoProductionsEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        AutoProductionsEnum::try_from(input.parse::<syn::Item>()?)
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
            let temps = (0..compiled_type.len())
                .map(|i| format_ident!("t{i}"))
                .collect_vec();
            let production = quote!(
                production!(#prod_ident: #ident -> (#(#compiled_type),*), |(#(#temps),*)| #ident::#variant_ident(#(#temps.into()),*));
            );
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
