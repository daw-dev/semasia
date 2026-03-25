use crate::{
    conflicts::{Associativity, ProductionPriority, TokenPriority},
    grammar::{Grammar, NonTerminal, Production, Symbol, Token},
};
use syn::Ident;

pub type EnrichedToken = Token<Ident, (Vec<syn::Attribute>, TokenPriority, Associativity)>;

pub type EnrichedNonTerminal = NonTerminal<Ident>;

pub type EnrichedSymbol = Symbol<EnrichedToken, EnrichedNonTerminal>;

pub struct Context(pub Option<Ident>);

pub type EnrichedBaseProduction = Production<Ident, Ident, Ident, Option<usize>>;

pub type EnrichedProduction =
    Production<Ident, EnrichedNonTerminal, EnrichedSymbol, ProductionPriority>;

pub type EnrichedGrammar = Grammar<EnrichedToken, EnrichedNonTerminal, EnrichedProduction, Context>;
