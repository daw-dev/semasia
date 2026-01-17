use dyn_grammar::production::EnrichedBaseProduction;
use syn::{ExprClosure, Ident, Token, braced, parenthesized, parse::Parse, punctuated::Punctuated};

#[derive(Debug)]
pub enum EbnfBodyItem {
    Ident(Ident),
    Alternatives {
        enum_ident: Ident,
        variants_types: Vec<Ident>,
    },
    Repetition(Ident),
    Optional(Ident),
}

impl Parse for EbnfBodyItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if input.peek(syn::token::Brace) {
            let variants_content;
            braced!(variants_content in input);
            let variants: Punctuated<Ident, Token![|]> =
                variants_content.parse_terminated(Ident::parse, Token![|])?;
            Ok(Self::Alternatives {
                enum_ident: ident,
                variants_types: variants.into_iter().collect(),
            })
        } else if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Ok(Self::Optional(ident))
        } else if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(Self::Repetition(ident))
        } else if input.is_empty() {
            Ok(Self::Ident(ident))
        } else {
            Err(syn::Error::new(
                ident.span(),
                "unexpected token in ebnf item",
            ))
        }
    }
}

#[derive(Debug)]
pub struct EbnfProduction {
    pub ident: Ident,
    pub head: Ident,
    pub body: Vec<EbnfBodyItem>,
    pub sem_action: Option<ExprClosure>,
}

impl EbnfProduction {
    fn new(
        ident: Ident,
        head: Ident,
        body: Vec<EbnfBodyItem>,
        sem_action: Option<ExprClosure>,
    ) -> Self {
        EbnfProduction {
            ident,
            head,
            body,
            sem_action,
        }
    }

    pub fn into_extra_productions(self) -> Vec<EnrichedBaseProduction> {
        let ebnf_production_name = self.ident;
        let mut res = Vec::with_capacity(self.body.len());
        let mut ebnf_computed_body = Vec::new();
        let head = self.head;
        for (item_index, item) in self.body.into_iter().enumerate() {
            match item {
                EbnfBodyItem::Alternatives {
                    enum_ident,
                    variants_types,
                } => {
                    for variant in variants_types.into_iter() {
                        res.push(EnrichedBaseProduction::new(
                            Self::alternative_ident(&ebnf_production_name, item_index, &variant),
                            enum_ident.clone(),
                            vec![variant],
                        ));
                    }
                    ebnf_computed_body.push(enum_ident);
                }
                EbnfBodyItem::Repetition(ident) => {
                    let rep_ident = Self::repetition_alias(&ebnf_production_name, item_index);
                    res.push(EnrichedBaseProduction::new(
                        Self::repetition_more_ident(&ebnf_production_name, item_index),
                        rep_ident.clone(),
                        vec![rep_ident.clone(), ident.clone()],
                    ));
                    res.push(EnrichedBaseProduction::new(
                        Self::repetition_done_ident(&ebnf_production_name, item_index),
                        rep_ident.clone(),
                        Vec::new(),
                    ));
                    ebnf_computed_body.push(rep_ident);
                }
                EbnfBodyItem::Optional(ident) => {
                    let opt_ident = Self::optional_alias(&ebnf_production_name, item_index);
                    res.push(EnrichedBaseProduction::new(
                        Self::optional_some_ident(&ebnf_production_name, item_index),
                        opt_ident.clone(),
                        vec![ident.clone()],
                    ));
                    res.push(EnrichedBaseProduction::new(
                        Self::optional_none_ident(&ebnf_production_name, item_index),
                        opt_ident.clone(),
                        Vec::new(),
                    ));
                    ebnf_computed_body.push(opt_ident);
                }
                EbnfBodyItem::Ident(ident) => {
                    ebnf_computed_body.push(ident);
                }
            }
        }
        res.push(EnrichedBaseProduction::new(
            ebnf_production_name,
            head,
            ebnf_computed_body,
        ));
        res
    }

    pub fn alternative_ident(
        ebnf_production_name: &Ident,
        item_index: usize,
        variant_ident: &Ident,
    ) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}{variant_ident}"),
            ebnf_production_name.span(),
        )
    }

    pub fn repetition_alias(ebnf_production_name: &Ident, item_index: usize) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}Rep"),
            ebnf_production_name.span(),
        )
    }

    pub fn repetition_more_ident(ebnf_production_name: &Ident, item_index: usize) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}More"),
            ebnf_production_name.span(),
        )
    }

    pub fn repetition_done_ident(ebnf_production_name: &Ident, item_index: usize) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}Done"),
            ebnf_production_name.span(),
        )
    }

    pub fn optional_alias(ebnf_production_name: &Ident, item_index: usize) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}Opt"),
            ebnf_production_name.span(),
        )
    }

    pub fn optional_some_ident(ebnf_production_name: &Ident, item_index: usize) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}Some"),
            ebnf_production_name.span(),
        )
    }

    pub fn optional_none_ident(ebnf_production_name: &Ident, item_index: usize) -> Ident {
        Ident::new(
            &format!("__{ebnf_production_name}{item_index}None"),
            ebnf_production_name.span(),
        )
    }
}

impl Parse for EbnfProduction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let head = input.parse()?;
        input.parse::<Token![->]>()?;
        let body = if input.peek(syn::token::Paren) {
            let body_content;
            parenthesized!(body_content in input);
            let body: Punctuated<EbnfBodyItem, Token![,]> =
                body_content.parse_terminated(EbnfBodyItem::parse, Token![,])?;
            body.into_iter().collect()
        } else {
            vec![input.parse()?]
        };

        if input.is_empty() {
            return Ok(EbnfProduction::new(ident, head, body, None));
        }

        input.parse::<Token![,]>()?;
        let sem_action = input.parse::<ExprClosure>()?;

        Ok(EbnfProduction::new(ident, head, body, Some(sem_action)))
    }
}
