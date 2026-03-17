use dyn_grammar::{EnrichedGrammar, lalr::LalrAutomaton, parsing::tables::{EofTable, NonTerminalTable, TokenTable}, symbolic_grammar::SymbolicGrammar};

pub struct Constructor;

pub struct Extracted {
    pub grammar: EnrichedGrammar,
}

pub struct Simplified {
    pub grammar: SymbolicGrammar,
}

pub struct Analyzed<'a> {
    pub automaton: LalrAutomaton<'a>,
    pub token_table: TokenTable,
    pub eof_table: EofTable,
    pub non_terminal_table: NonTerminalTable,
}
