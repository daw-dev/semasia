use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    NonTerminal(usize),
    Token(usize),
    EOF,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Token(token) => write!(f, "`{}`", token),
            Symbol::NonTerminal(non_terminal) => write!(f, "{}", non_terminal),
            Symbol::EOF => write!(f, "$"),
        }
    }
}
