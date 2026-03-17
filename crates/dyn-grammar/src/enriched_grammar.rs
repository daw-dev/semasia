use crate::{conflicts::{Associativity, Precedence}, grammar::{Grammar, NonTerminal, Production, Symbol, Token}};
use std::fmt::Display;
use syn::Ident;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Match {
    Literal(String),
    Regex(String),
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => write!(f, "\"{lit}\""),
            Self::Regex(reg) => write!(f, "/{reg}/"),
        }
    }
}

pub type EnrichedToken = Token<Ident, (Match, Precedence, Associativity)>;

pub type EnrichedNonTerminal = NonTerminal<Ident>;

pub type EnrichedSymbol = Symbol<EnrichedToken, EnrichedNonTerminal>;

pub struct Context(pub Option<Ident>);

pub type EnrichedBaseProduction = Production<Ident, Ident, Ident, Option<usize>>;

pub type EnrichedProduction = Production<Ident, EnrichedNonTerminal, EnrichedSymbol, Precedence>;

pub type EnrichedGrammar = Grammar<EnrichedToken, EnrichedNonTerminal, EnrichedProduction, Context>;
