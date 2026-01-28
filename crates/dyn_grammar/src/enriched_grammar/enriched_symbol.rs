use std::fmt::Display;

use crate::{non_terminal::EnrichedNonTerminal, token::EnrichedToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnrichedSymbol {
    Token(EnrichedToken),
    NonTerminal(EnrichedNonTerminal),
}

impl Display for EnrichedSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(tok) => tok.fmt(f),
            Self::NonTerminal(nt) => nt.fmt(f),
        }
    }
}
