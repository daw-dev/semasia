use std::fmt::Display;

use logos::Logos;

use crate::{EofAction, Parser, Reduce, Stacks, Tables, TokenAction};

#[derive(Debug)]
pub struct DummyNonTerminal;

impl Display for DummyNonTerminal {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

#[derive(Logos, Debug)]
pub enum DummyToken {}

impl Display for DummyToken {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

#[derive(Debug)]
pub struct DummyStartSymbol;

impl Display for DummyStartSymbol {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

impl From<DummyNonTerminal> for DummyStartSymbol {
    fn from(_value: DummyNonTerminal) -> Self {
        unreachable!()
    }
}

#[derive(Debug)]
pub struct DummyProductions;

#[derive(Debug)]
pub struct DummyTable;

pub type DummyParser = Parser<DummyNonTerminal, DummyToken, DummyStartSymbol, DummyProductions, DummyTable, ()>;

impl Tables<DummyNonTerminal, DummyToken, DummyProductions> for DummyTable {
    fn query_token_table(_current_state: usize, _current_token: &DummyToken) -> Option<TokenAction<DummyProductions>> {
        unreachable!()
    }

    fn query_eof_table(_current_state: usize) -> Option<EofAction<DummyProductions>> {
        unreachable!()
    }

    fn query_goto_table(_current_state: usize, _non_terminal: &DummyNonTerminal) -> Option<usize> {
        unreachable!()
    }

    fn tokens_in_state(_current_state: usize) -> &'static [&'static str] {
        unreachable!()
    }
}

impl Reduce<DummyNonTerminal, DummyToken, ()> for DummyProductions {
    fn reduce(&self, _ctx: &mut (), _stacks: &mut Stacks<DummyNonTerminal, DummyToken>) -> DummyNonTerminal {
        unreachable!()
    }
}
