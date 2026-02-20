use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{
    non_terminal,
    parsing::action::{EofAction, TokenAction},
    symbolic_grammar::{SymbolicNonTerminal, SymbolicSymbol, SymbolicToken},
};

#[derive(Debug)]
pub struct TokenTable {
    tokens_count: usize,
    pub table: Vec<Vec<Option<TokenAction>>>,
}

impl TokenTable {
    pub fn new(tokens_count: usize) -> Self {
        Self {
            tokens_count,
            table: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        let state_id = self.table.len();
        self.table.push(vec![None; self.tokens_count]);
        state_id
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.tokens_count + 1, self.table.len())
    }
}

impl Index<(usize, SymbolicToken)> for TokenTable {
    type Output = Option<TokenAction>;

    fn index(&self, (state, token): (usize, SymbolicToken)) -> &Self::Output {
        &self.table[state][token]
    }
}

impl IndexMut<(usize, SymbolicToken)> for TokenTable {
    fn index_mut(&mut self, (state, token): (usize, SymbolicToken)) -> &mut Self::Output {
        &mut self.table[state][token]
    }
}

impl Display for TokenTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(5))?;
        for i in 0..self.tokens_count {
            write!(f, "{:^5}", i)?;
        }
        writeln!(f)?;
        for (state, row) in self.table.iter().enumerate() {
            write!(f, "{:^5}", state)?;
            for elem in row.iter() {
                match elem {
                    Some(target) => {
                        write!(
                            f,
                            "{:^5}",
                            match target {
                                TokenAction::Shift(state) => format!("S{state}"),
                                TokenAction::Reduce(id) => format!("R{id}"),
                            }
                        )
                    }
                    None => write!(f, "{}", " ".repeat(5)),
                }?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct EofTable {
    pub table: Vec<Option<EofAction>>,
}

impl EofTable {
    pub fn new() -> Self {
        Self { table: Vec::new() }
    }

    pub fn add_state(&mut self) -> usize {
        let state_id = self.table.len();
        self.table.push(None);
        state_id
    }
}

impl Index<usize> for EofTable {
    type Output = Option<EofAction>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl IndexMut<usize> for EofTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.table[index]
    }
}

impl Display for EofTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(5))?;
        writeln!(f, "  $  ")?;
        for (state, action) in self.table.iter().enumerate() {
            write!(f, "{:^5}", state)?;
            match action {
                Some(action) => write!(
                    f,
                    "{:^5}",
                    match action {
                        EofAction::Reduce(id) => format!("R{id}"),
                        EofAction::Accept => "Acc".to_string(),
                    }
                ),
                None => write!(f, "{}", " ".repeat(5)),
            }?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct NonTerminalTable {
    non_terminals_count: usize,
    pub table: Vec<Vec<Option<usize>>>,
}

impl NonTerminalTable {
    pub fn new(non_terminals_count: usize) -> Self {
        Self {
            non_terminals_count,
            table: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        let state_id = self.table.len();
        self.table.push(vec![None; self.non_terminals_count]);
        state_id
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.non_terminals_count, self.table.len())
    }
}

impl Index<(usize, SymbolicNonTerminal)> for NonTerminalTable {
    type Output = Option<usize>;

    fn index(&self, (state, non_terminal): (usize, SymbolicNonTerminal)) -> &Self::Output {
        &self.table[state][non_terminal]
    }
}

impl IndexMut<(usize, SymbolicNonTerminal)> for NonTerminalTable {
    fn index_mut(
        &mut self,
        (state, non_terminal): (usize, SymbolicNonTerminal),
    ) -> &mut Self::Output {
        &mut self.table[state][non_terminal]
    }
}

impl Display for NonTerminalTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(5))?;
        for i in 0..self.non_terminals_count {
            write!(f, "{:^5}", i)?;
        }
        writeln!(f)?;
        for (state, row) in self.table.iter().enumerate() {
            write!(f, "{:^5}", state)?;
            for elem in row.iter() {
                match elem {
                    Some(target) => write!(f, "{:^5}", target),
                    None => write!(f, "{}", " ".repeat(5)),
                }?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct TransitionTables {
    token_table: Vec<Vec<Option<usize>>>,
    non_terminal_table: Vec<Vec<Option<usize>>>,
}

impl TransitionTables {
    pub fn new() -> Self {
        Self {
            token_table: Vec::new(),
            non_terminal_table: Vec::new(),
        }
    }

    pub fn add_transitions(
        &mut self,
        token_transitions: Vec<Option<usize>>,
        non_terminal_transitions: Vec<Option<usize>>,
    ) {
        self.token_table.push(token_transitions);
        self.non_terminal_table.push(non_terminal_transitions);
    }

    pub fn token_transition(&self, starting_state: usize, token: SymbolicToken) -> Option<usize> {
        self.token_table
            .get(starting_state)?
            .get(token)
            .cloned()
            .flatten()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Vec<Option<usize>>, &Vec<Option<usize>>)> {
        self.token_table.iter().zip(self.non_terminal_table.iter())
    }
}

impl Display for TransitionTables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TransitionTables:")?;
        write!(f, "{}", " ".repeat(5))?;
        for i in 0..self.non_terminal_table[0].len() {
            write!(f, "{:^5}", i)?;
        }
        for i in 0..self.token_table[0].len() {
            write!(f, "{:^5}", format!("`{i}`"))?;
        }
        writeln!(f)?;
        for (state, (nt_row, tok_row)) in self
            .non_terminal_table
            .iter()
            .zip(self.token_table.iter())
            .enumerate()
        {
            write!(f, "{:^5}", state)?;
            for elem in nt_row.iter() {
                match elem {
                    Some(target) => write!(f, "{:^5}", target),
                    None => write!(f, "{}", " ".repeat(5)),
                }?
            }
            for elem in tok_row.iter() {
                match elem {
                    Some(target) => write!(f, "{:^5}", target),
                    None => write!(f, "{}", " ".repeat(5)),
                }?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
