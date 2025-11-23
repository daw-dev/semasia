use std::fmt::Display;
use crate::{non_terminal::NonTerminal, token::Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    NonTerminal(NonTerminal),
    Token(Token),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Token(token) => write!(f, "`{}`", token),
            Symbol::NonTerminal(non_terminal) => write!(f, "{}", non_terminal),
        }
    }
}
