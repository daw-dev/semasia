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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnrichedToken {
    ident: Ident,
    match_string: Match,
}

impl EnrichedToken {
    pub fn new(ident: Ident, match_string: Match) -> Self {
        Self {
            ident,
            match_string,
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn match_string(&self) -> &Match {
        &self.match_string
    }
}

impl Display for EnrichedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.match_string, self.ident)
    }
}
