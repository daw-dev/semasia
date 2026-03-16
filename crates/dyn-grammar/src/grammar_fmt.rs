use itertools::Itertools;
use std::fmt::Display;

use crate::{
    EnrichedGrammar, EnrichedNonTerminal, EnrichedToken,
    production::{EnrichedBaseProduction, EnrichedProduction},
    symbolic_grammar::{SymbolicNonTerminal, SymbolicProduction, SymbolicToken},
};

impl Display for EnrichedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id().fmt(f)
    }
}

impl Display for SymbolicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.extras().fmt(f)
    }
}

impl Display for EnrichedNonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id().fmt(f)
    }
}

impl Display for SymbolicNonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.extras().fmt(f)
    }
}

impl Display for EnrichedProduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} -> ({})",
            self.id(),
            self.head(),
            self.body().iter().format(", ")
        )
    }
}

impl Display for SymbolicProduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} -> ({})",
            self.extras(),
            self.head(),
            self.body().iter().format(", ")
        )
    }
}

impl Display for EnrichedGrammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnrichedGrammar")
            .field(
                "tokens",
                &format!("[{}]", self.tokens().iter().format(", ")),
            )
            .field(
                "non_terminals",
                &format!("[{}]", self.non_terminals().iter().format(", ")),
            )
            .field(
                "productions",
                &format!("[{}]", self.productions().iter().format(", ")),
            )
            .finish()
    }
}
