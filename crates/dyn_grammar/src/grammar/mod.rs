pub mod non_terminal;
pub mod production;
pub mod symbol;
pub mod token;

use crate::{non_terminal::NonTerminal, production::Production, symbol::Symbol, token::Token};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct Grammar {
    non_terminals: Vec<NonTerminal>,
    tokens: Vec<Token>,
    productions: Vec<Production>,
    symbols_map: HashMap<String, usize>,
    productions_map: HashMap<String, usize>,
}

impl Grammar {
    pub fn new(
        non_terminals: Vec<NonTerminal>,
        tokens: Vec<Token>,
        productions: Vec<Production>,
    ) -> Self {
        let symbols_map = Self::compute_symbols_map(&non_terminals, &tokens);
        Self {
            non_terminals,
            tokens,
            productions,
            symbols_map,
        }
    }

    fn compute_symbols_map(
        non_terminals: &Vec<NonTerminal>,
        tokens: &Vec<Token>,
    ) -> HashMap<String, usize> {
        std::iter::chain(
            tokens.iter().map(|token| token.name().clone()),
            non_terminals.iter().map(|non_t| non_t.name().clone()),
        )
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect()
    }

    fn compute_productions_map(productions: &Vec<Production>) -> HashMap<String, usize> {
        productions
            .iter()
            .enumerate()
            .map(|(i, prod)| (prod.name().clone(), i))
            .collect()
    }

    pub fn get_symbol(&self, name: &String) -> Option<Symbol> {
        let index = *self.symbols_map.get(name)?;
        if index < self.tokens.len() {
            self.tokens.get(index).cloned().map(Symbol::Token)
        } else {
            self.non_terminals
                .get(index)
                .cloned()
                .map(Symbol::NonTerminal)
        }
    }

    pub fn get_production(&self, name: &String) -> Option<&Production> {
        self.productions_map.get(name).map(|i| &self.productions[*i])
    }
}
