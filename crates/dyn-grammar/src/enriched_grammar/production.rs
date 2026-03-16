use crate::{
    enriched_grammar::EnrichedSymbol, enriched_grammar::EnrichedNonTerminal, enriched_grammar::EnrichedToken,
    grammar::Production,
};
use syn::Ident;

pub type EnrichedBaseProduction = Production<Ident, Ident, Ident>;

pub type EnrichedProduction = Production<Ident, EnrichedNonTerminal, EnrichedSymbol>;
