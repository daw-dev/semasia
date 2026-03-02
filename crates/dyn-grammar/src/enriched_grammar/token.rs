use std::fmt::Display;
use syn::Ident;

use crate::conflicts::{Associativity, Precedence};

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnrichedToken {
    ident: Ident,
    match_string: Match,
    precedence: Precedence,
    associativity: Associativity,
}

impl EnrichedToken {
    pub fn new(ident: Ident, match_string: Match, precedence: Precedence, associativity: Associativity) -> Self {
        Self {
            ident,
            match_string,
            precedence,
            associativity,
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn match_string(&self) -> &Match {
        &self.match_string
    }

    pub fn precedence(&self) -> &Precedence {
        &self.precedence
    }

    pub fn associativity(&self) -> &Associativity {
        &self.associativity
    }
}

impl Display for EnrichedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.match_string, self.ident)
    }
}
