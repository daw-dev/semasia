use crate::parsing::action::Action;

pub struct ActionTable {
    tokens_count: usize,
    table: Vec<Vec<Option<Action>>>,
}

impl ActionTable {
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
        (self.table.len(), self.tokens_count)
    }
}

pub struct GoToTable {
    non_terminals_count: usize,
    table: Vec<Vec<Option<usize>>>,
}

impl GoToTable {
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
        (self.table.len(), self.non_terminals_count)
    }
}
