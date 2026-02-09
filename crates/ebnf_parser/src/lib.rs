use dyn_grammar::production::EnrichedBaseProduction;
use itertools::Itertools;
use proc_macro2::{self, Span};
use std::fmt::Display;
use syn::{ExprClosure, Ident, Token, braced, parenthesized, parse::Parse, parse_quote, spanned};

#[derive(Debug)]
pub struct EbnfAlternativeVariant {
    ident: Ident,
    v_type: EbnfBody,
}

impl Parse for EbnfAlternativeVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let body = if input.peek(syn::token::Paren) {
            input.parse()?
        } else {
            EbnfBody {
                items: vec![EbnfBodyItem::Ident(ident.clone())],
            }
        };

        Ok(EbnfAlternativeVariant {
            ident,
            v_type: body,
        })
    }
}

impl Display for EbnfAlternativeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.ident, self.v_type)
    }
}

impl EbnfAlternativeVariant {
    fn compile_helper(
        self,
        enum_ident: Ident,
        res: &mut Vec<EbnfCompiledProduction>,
        id_stack: &mut Vec<String>,
    ) {
        id_stack.push(self.ident.to_string());

        let compiled_body = self.v_type.compile_helper(res, id_stack);

        res.push(EbnfCompiledProduction {
            ident: EbnfBodyItem::compose_name(id_stack, Span::call_site()),
            head: enum_ident,
            body: compiled_body,
            sem_action: CompiledSemAction::Alternative,
        });

        id_stack.pop();
    }
}

#[derive(Debug)]
pub enum EbnfBodyItem {
    Ident(Ident),
    Alternatives {
        enum_ident: Ident,
        variants_types: Vec<EbnfAlternativeVariant>,
    },
    Repetition(EbnfBody),
    Optional(EbnfBody),
}

impl EbnfBodyItem {
    fn compile_helper(
        self,
        res: &mut Vec<EbnfCompiledProduction>,
        id_stack: &mut Vec<String>,
        span: Span,
    ) -> Ident {
        match self {
            EbnfBodyItem::Alternatives {
                enum_ident,
                variants_types,
            } => {
                id_stack.push(enum_ident.to_string());

                for variant in variants_types.into_iter() {
                    variant.compile_helper(enum_ident.clone(), res, id_stack);
                }
                enum_ident
            }
            EbnfBodyItem::Repetition(body) => {
                let rep_ident = Self::repetition_alias(id_stack, span);

                let compiled_body = body.compile_helper(res, id_stack);

                res.push(EbnfCompiledProduction::new(
                    Self::repetition_more_ident(id_stack, span),
                    rep_ident.clone(),
                    std::iter::once(rep_ident.clone())
                        .chain(compiled_body.into_iter())
                        .collect(),
                    CompiledSemAction::RepetitionMore,
                ));

                res.push(EbnfCompiledProduction::new(
                    Self::repetition_done_ident(id_stack, span),
                    rep_ident.clone(),
                    Vec::with_capacity(0),
                    CompiledSemAction::RepetitionDone,
                ));

                rep_ident
            }
            EbnfBodyItem::Optional(body) => {
                let opt_ident = Self::repetition_alias(id_stack, span);

                let compiled_body = body.compile_helper(res, id_stack);

                res.push(EbnfCompiledProduction::new(
                    Self::optional_some_ident(id_stack, span),
                    opt_ident.clone(),
                    compiled_body,
                    CompiledSemAction::OptionalSome,
                ));

                res.push(EbnfCompiledProduction::new(
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
        Ident::new(&id_stack.iter().format("_").to_string(), span)
    }

    pub fn repetition_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Rep".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    pub fn repetition_more_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("More".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    pub fn repetition_done_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Done".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    pub fn optional_alias(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Opt".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    pub fn optional_some_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("Some".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }

    pub fn optional_none_ident(id_stack: &mut Vec<String>, span: Span) -> Ident {
        id_stack.push("None".to_string());
        let res = Self::compose_name(id_stack, span);
        id_stack.pop();
        res
    }
}

impl Parse for EbnfBodyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let elements: EbnfBody = input.parse()?;

            if elements.items.is_empty() {
                Err(syn::Error::new(
                    Span::call_site(),
                    "cannot use empty parenthesis",
                ))
            } else if input.peek(Token![?]) {
                input.parse::<Token![?]>()?;
                Ok(Self::Optional(elements))
            } else if input.peek(Token![*]) {
                input.parse::<Token![*]>()?;
                Ok(Self::Repetition(elements))
            } else {
                Err(syn::Error::new(
                    Span::call_site(),
                    "cannot use parenthesis without * or ?",
                ))
            }
        } else {
            let ident: Ident = input.parse()?;
            if input.peek(syn::token::Brace) {
                let variants_content;
                braced!(variants_content in input);
                let variants =
                    variants_content.parse_terminated(EbnfAlternativeVariant::parse, Token![,])?;
                let variants_types = variants.into_iter().collect();
                Ok(Self::Alternatives {
                    enum_ident: ident,
                    variants_types,
                })
            } else if input.peek(Token![?]) {
                input.parse::<Token![?]>()?;
                Ok(Self::Optional(EbnfBody {
                    items: vec![EbnfBodyItem::Ident(ident)],
                }))
            } else if input.peek(Token![*]) {
                input.parse::<Token![*]>()?;
                Ok(Self::Repetition(EbnfBody {
                    items: vec![EbnfBodyItem::Ident(ident)],
                }))
            } else {
                Ok(Self::Ident(ident))
            }
        }
    }
}

impl Display for EbnfBodyItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EbnfBodyItem::Ident(ident) => write!(f, "{ident}"),
            EbnfBodyItem::Alternatives {
                enum_ident,
                variants_types,
            } => write!(
                f,
                "{enum_ident} {{ {} }}",
                variants_types.iter().format(", ")
            ),
            EbnfBodyItem::Repetition(body) => {
                write!(f, "{body}*")
            }
            EbnfBodyItem::Optional(body) => {
                write!(f, "{body}?")
            }
        }
    }
}

#[derive(Debug)]
pub struct EbnfBody {
    items: Vec<EbnfBodyItem>,
}

impl EbnfBody {
    fn compile_helper(
        self,
        res: &mut Vec<EbnfCompiledProduction>,
        id_stack: &mut Vec<String>,
    ) -> Vec<Ident> {
        let body = self
            .items
            .into_iter()
            .enumerate()
            .map(|(item_index, item)| {
                id_stack.push(item_index.to_string());
                let ident = item.compile_helper(res, id_stack, Span::call_site());
                id_stack.pop();
                ident
            })
            .collect();

        body
    }
}

impl Parse for EbnfBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items = if input.peek(syn::token::Paren) {
            let body_content;
            parenthesized!(body_content in input);
            let body = body_content.parse_terminated(EbnfBodyItem::parse, Token![,])?;
            body.into_iter().collect()
        } else {
            vec![input.parse()?]
        };

        Ok(EbnfBody { items })
    }
}

impl Display for EbnfBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.items.as_slice() {
            [only_item] => write!(f, "{only_item}"),
            other => write!(f, "({})", other.iter().format(", ")),
        }
    }
}

#[derive(Debug)]
pub enum CompiledSemAction {
    Alternative,
    RepetitionMore,
    RepetitionDone,
    OptionalSome,
    OptionalNone,
    Compiled,
}

#[derive(Debug)]
pub struct EbnfCompiledProduction {
    pub ident: Ident,
    pub head: Ident,
    pub body: Vec<Ident>,
    pub sem_action: CompiledSemAction,
}

impl EbnfCompiledProduction {
    pub fn new(ident: Ident, head: Ident, body: Vec<Ident>, sem_action: CompiledSemAction) -> Self {
        Self {
            ident,
            head,
            body,
            sem_action,
        }
    }
}

impl Into<EnrichedBaseProduction> for EbnfCompiledProduction {
    fn into(self) -> EnrichedBaseProduction {
        EnrichedBaseProduction::new(self.ident, self.head, self.body)
    }
}

#[derive(Debug)]
pub struct EbnfProduction {
    pub ident: Ident,
    pub head: Ident,
    pub body: EbnfBody,
    pub sem_action: Option<ExprClosure>,
}

impl EbnfProduction {
    fn new(ident: Ident, head: Ident, body: EbnfBody, sem_action: Option<ExprClosure>) -> Self {
        EbnfProduction {
            ident,
            head,
            body,
            sem_action,
        }
    }

    pub fn compile(self) -> Vec<EbnfCompiledProduction> {
        let ident = self.ident;
        let mut res = Vec::with_capacity(self.body.items.len());
        let head = self.head;

        let compiled_body = self
            .body
            .compile_helper(&mut res, &mut vec!["_".to_string(), ident.to_string()]);

        res.push(EbnfCompiledProduction {
            ident,
            head,
            body: compiled_body,
            sem_action: CompiledSemAction::Compiled,
        });

        res
    }
}

impl Parse for EbnfProduction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let head = input.parse()?;
        input.parse::<Token![->]>()?;
        let body = input.parse()?;

        if input.is_empty() {
            return Ok(EbnfProduction::new(ident, head, body, None));
        }

        input.parse::<Token![,]>()?;
        let sem_action = input.parse::<ExprClosure>()?;

        Ok(EbnfProduction::new(ident, head, body, Some(sem_action)))
    }
}

impl Display for EbnfProduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} -> {}", self.ident, self.head, self.body,)
    }
}

#[test]
fn ebnf_test() {
    let ebnf: EbnfProduction =
        parse_quote!(Test, A -> ((A, B)?, (CDorEStarOrF { CD(C, D), EStar(E*), F })?));
    println!("debug print:");
    println!("{ebnf:?}");
    println!("pretty print:");
    println!("{ebnf}");

    println!(
        "{}",
        ebnf.compile()
            .into_iter()
            .map(|prod| Into::<EnrichedBaseProduction>::into(prod))
            .format("\n")
    );
}
