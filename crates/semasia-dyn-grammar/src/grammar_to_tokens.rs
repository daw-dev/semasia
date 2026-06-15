use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::grammar::{Body, Production};

impl<BodySymbol> ToTokens for Body<BodySymbol>
where
    BodySymbol: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self[..] {
            [one] => one.to_tokens(tokens),
            more => {
                tokens.extend(quote!((#(#more),*)));
            }
        }
    }
}

impl<ProductionId, HeadType, BodySymbol, Extras> ToTokens
    for Production<ProductionId, HeadType, BodySymbol, Option<Extras>>
where
    ProductionId: ToTokens,
    HeadType: ToTokens,
    Body<BodySymbol>: ToTokens,
    Extras: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.id().to_tokens(tokens);
        tokens.extend(quote!(:));
        self.head().to_tokens(tokens);
        tokens.extend(quote!(->));
        self.body().to_tokens(tokens);
        if let Some(extra) = self.extras() {
            tokens.extend(quote!(,));
            extra.to_tokens(tokens);
        }
    }
}

