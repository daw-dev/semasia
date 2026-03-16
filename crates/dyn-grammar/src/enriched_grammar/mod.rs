pub mod production;

use crate::{
    grammar::{Grammar, NonTerminal, Symbol, Token},
    production::EnrichedProduction,
};
use std::fmt::Display;
use syn::Ident;

// #[derive(Debug)]
// pub struct EnrichedGrammar {
//     context: Option<Ident>,
//     non_terminals: Vec<EnrichedNonTerminal>,
//     tokens: Vec<EnrichedToken>,
//     start_symbol: EnrichedNonTerminal,
//     productions: Vec<EnrichedProduction>,
// }

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

pub type EnrichedToken = Token<Ident, Match>;

pub type EnrichedNonTerminal = NonTerminal<Ident>;

pub type EnrichedSymbol = Symbol<EnrichedToken, EnrichedNonTerminal>;

pub struct Context(Option<Ident>);

pub type EnrichedGrammar = Grammar<EnrichedToken, EnrichedNonTerminal, EnrichedProduction, Context>;
