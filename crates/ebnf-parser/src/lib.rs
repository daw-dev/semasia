use dyn_grammar::grammar::{Body, Production};
use itertools::Itertools;
use proc_macro_error::{emit_error, emit_warning};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ExprClosure, Ident, Token, Type, TypeTuple, parse::Parse};

impl EbnfBodyItem {
    fn compile_helper(
        self,
        productions: &mut Vec<EbnfCompiledProduction>,
        types: &mut Vec<EbnfCompiledType>,
        id_stack: &mut Vec<String>,
        span: Span,
    ) -> Ident {
        match self {
            EbnfBodyItem::Alternatives {
                enum_ident,
                variants_types,
            } => {
                id_stack.push(enum_ident.to_string());

                let variants = variants_types
                    .into_iter()
                    .map(|variant| {
                        (
                            variant.ident.clone(),
                            variant.compile_helper(
                                enum_ident.clone(),
                                productions,
                                types,
                                id_stack,
                                span,
                            ),
                        )
                    })
                    .collect_vec();

                types.push(EbnfCompiledType::Enum {
                    enum_ident: enum_ident.clone(),
                    enum_variants: variants,
                });

                id_stack.pop();

                enum_ident
            }
            EbnfBodyItem::Repetition(body, separator) => {
                let rep_ident = Self::repetition_alias(id_stack, span);

                let compiled_body = body.compile_helper(productions, types, id_stack, span);

                let compiled_body_len = compiled_body.len();

                types.push(EbnfCompiledType::Repetition {
                    alias_ident: rep_ident.clone(),
                    alias_vec_types: compiled_body.clone(),
                });

                match separator {
                    Some(separator) => {
                        let non_empty_rep_alias = Self::non_empty_repetition_alias(id_stack, span);

                        types.push(EbnfCompiledType::Repetition {
                            alias_ident: non_empty_rep_alias.clone(),
                            alias_vec_types: compiled_body.clone(),
                        });

                        let separator_body =
                            separator.compile_helper(productions, types, id_stack, span);

                        productions.push(EbnfCompiledProduction::new(
                            Self::repetition_non_empty_ident(id_stack, span),
                            rep_ident.clone(),
                            vec![non_empty_rep_alias.clone()],
                            CompiledSemAction::RepetitionNonEmpty,
                        ));

                        productions.push(EbnfCompiledProduction::new(
                            Self::repetition_empty_ident(id_stack, span),
                            rep_ident.clone(),
                            Vec::with_capacity(0),
                            CompiledSemAction::RepetitionEmpty,
                        ));

                        productions.push(EbnfCompiledProduction::new(
                            Self::repetition_more_ident(id_stack, span),
                            non_empty_rep_alias.clone(),
                            std::iter::once(non_empty_rep_alias.clone())
                                .chain(separator_body.into_iter())
                                .chain(compiled_body.clone())
                                .collect(),
                            CompiledSemAction::RepetitionSepMore(compiled_body_len),
                        ));

                        productions.push(EbnfCompiledProduction::new(
                            Self::repetition_done_ident(id_stack, span),
                            non_empty_rep_alias.clone(),
                            compiled_body,
                            CompiledSemAction::RepetitionSepDone,
                        ));
                    }
                    None => {
                        productions.push(EbnfCompiledProduction::new(
                            Self::repetition_more_ident(id_stack, span),
                            rep_ident.clone(),
                            std::iter::once(rep_ident.clone())
                                .chain(compiled_body)
                                .collect(),
                            CompiledSemAction::SimpleRepetitionMore,
                        ));

                        productions.push(EbnfCompiledProduction::new(
                            Self::repetition_done_ident(id_stack, span),
                            rep_ident.clone(),
                            Vec::with_capacity(0),
                            CompiledSemAction::SimpleRepetitionDone,
                        ));
                    }
                }

                rep_ident
            }
            EbnfBodyItem::Optional(body) => {
                let opt_ident = Self::optional_alias(id_stack, span);

                let compiled_body = body.compile_helper(productions, types, id_stack, span);

                types.push(EbnfCompiledType::Optional {
                    alias_ident: opt_ident.clone(),
                    alias_opt_types: compiled_body.clone(),
                });

                productions.push(EbnfCompiledProduction::new(
                    Self::optional_some_ident(id_stack, span),
                    opt_ident.clone(),
                    compiled_body,
                    CompiledSemAction::OptionalSome,
                ));

                productions.push(EbnfCompiledProduction::new(
                    Self::optional_none_ident(id_stack, span),
                    opt_ident.clone(),
                    Vec::with_capacity(0),
                    CompiledSemAction::OptionalNone,
                ));

                opt_ident
            }
            EbnfBodyItem::Ident(ident) => ident,
        }
    }

    fn compose_name(id_stack: &[String], span: Span) -> Ident {
        Ident::new(&id_stack.iter().format("").to_string(), span)
    }

    fn repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Rep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn non_empty_repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("NonEmptyRep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_more_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("More".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_done_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Done".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_empty_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Empty".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_non_empty_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("NonEmpty".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Opt".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_some_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Some".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_none_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("None".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }
}

pub type EbnfCompiledType = TokenStream;
pub type CompiledSemAction = TokenStream;
pub type EbnfCompiledProduction = Production<Ident, Ident, Ident, Option<ExprClosure>>;

struct RestrictedTypeArray {

}

struct RestrictedTypePath {

}

enum RestrictedType {
    Array(RestrictedTypeArray),
    Tuple(RestrictedTypeTuple),
    Path(RestrictedTypePath),
}

struct RestrictedTypeWrapper {
    attrs: Vec<syn::Attribute>,
    ty: RestrictedType,
}

struct RestrictedTypeTuple {
    paren_token: syn::token::Paren,
    elems: syn::punctuated::Punctuated<RestrictedTypeWrapper, syn::token::Comma>
}

impl Parse for RestrictedTypeTuple {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

pub struct EbnfProduction {
    ident: Ident,
    head: Ident,
    body: RestrictedTypeTuple,
    sem_action: Option<ExprClosure>,
}

impl EbnfProduction {
    fn compile_tuple(
        tuple: RestrictedTypeTuple,
        productions: &mut Vec<EbnfCompiledProduction>,
        types: &mut Vec<EbnfCompiledType>,
        id_stack: &mut Vec<String>,
        span: Span,
    ) -> Vec<Ident> {
        todo!()
    }

    fn compile_type(
        ty: RestrictedType,
        productions: &mut Vec<EbnfCompiledProduction>,
        types: &mut Vec<EbnfCompiledType>,
        id_stack: &mut Vec<String>,
        span: Span,
    ) -> Ident {
        match ty {
            RestrictedType::Path(type_path) => {
                let last_segment = type_path.path.segments.pop().unwrap().into_value();
                let ident = last_segment.ident;
                match last_segment.arguments {
                    syn::PathArguments::None => {
                        ident
                    }
                    syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                        let arg = args.args.pop().unwrap().into_value();
                        let syn::GenericArgument::Type(arg) = arg else {
                            emit_error!(arg, "argument has to be a type");
                            return ident;
                        };
                        if ident == "Vec" {
                            let alias = Self::repetition_alias(id_stack, span);
                            let inner_ident = match arg {
                                Type::Path(type_path) => {
                                    let Some(inner_ident) = type_path.path.get_ident() else {
                                        emit_error!(
                                            type_path,
                                            "argument of vec has to be either an ident or a tuple"
                                        );
                                        return ident;
                                    };
                                    inner_ident.clone()
                                }
                                Type::Tuple(type_tuple) => {
                                    let inner_ident = Self::compile_tuple(
                                        type_tuple,
                                        productions,
                                        types,
                                        id_stack,
                                        span,
                                    );
                                    inner_ident
                                }
                                _ => {
                                    emit_error!(
                                        arg,
                                        "argument of vec has to be either an ident or a tuple"
                                    );
                                    return ident;
                                }
                            };
                            
                            alias
                        } else if ident == "Option" {
                            let alias = Self::optional_alias(id_stack, span);
                            alias
                        // } else if args.args.len() == 1 {
                        //     emit_warning!(
                        //         ident,
                        //         "ident is neither Vec nor Option, so {arg} is used as symbol"
                        //     );
                        //     ident
                        } else {
                            emit_error!(args, "cannot have multiple generic arguments");
                            ident
                        }
                    }
                    syn::PathArguments::AngleBracketed(args) => {
                        emit_error!(
                            args,
                            "generic arguments different from <Type> are not valid in an ebnf item"
                        );
                        ident
                    }
                    syn::PathArguments::Parenthesized(args) => {
                        emit_error!(
                            args,
                            "parenthesized arguments are not valid in an ebnf item"
                        );
                        ident
                    }
                }
            }
            RestrictedType::Array(type_array) => todo!(),
            RestrictedType::Tuple(type_tuple) => todo!(),
            _ => todo!(),
        }
    }

    pub fn compile(self) -> (Vec<EbnfCompiledProduction>, Vec<EbnfCompiledType>) {
        let ident = self.ident;
        let mut productions = Vec::with_capacity(self.body.elems.len());
        let mut types = Vec::with_capacity(self.body.elems.len());
        let head = self.head;

        let compiled_body = Self::compile_tuple(
            self.body,
            &mut productions,
            &mut types,
            &mut vec!["__Ebnf".to_string(), ident.to_string()],
            ident.span().clone(),
        );

        productions.push(Production::new(ident, head, Body::new(compiled_body), self.sem_action));

        (productions, types)
    }

    fn compose_name(id_stack: &[String], span: Span) -> Ident {
        Ident::new(&id_stack.iter().format("").to_string(), span)
    }

    fn repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Rep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn non_empty_repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("NonEmptyRep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_more_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("More".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_done_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Done".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_empty_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Empty".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn repetition_non_empty_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("NonEmpty".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Opt".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_some_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Some".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    fn optional_none_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("None".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }
}

impl Parse for EbnfProduction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let head = input.parse()?;
        input.parse::<Token![->]>()?;
        let body = input.parse()?;

        if input.is_empty() {
            return Ok(EbnfProduction {
                ident,
                head,
                body,
                sem_action: None,
            });
        }

        input.parse::<Token![,]>()?;
        let sem_action = input.parse()?;

        Ok(EbnfProduction {
            ident,
            head,
            body,
            sem_action: Some(sem_action),
        })
    }
}
