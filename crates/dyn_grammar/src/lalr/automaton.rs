use crate::{
    Grammar,
    lalr::{equation::EquationSet, item::LalrItem},
};
use std::collections::HashSet;

pub struct LalrState {
    state_id: usize,
    kernel: HashSet<LalrItem>,
}

impl LalrState {
    pub fn new(state_id: usize, kernel: HashSet<LalrItem>) -> Self {
        Self { state_id, kernel }
    }

    fn closure(&self, grammar: &Grammar) -> HashSet<LalrItem> {
        let mut res = self.kernel.clone();
        res.iter().map(|item| {
            
        });
        res
    }
}

#[derive(Default)]
pub struct SymbolicAutomaton {
    states: Vec<LalrState>,
    symbols_count: usize,
    transitions: Vec<Vec<Option<usize>>>,
    equations: EquationSet,
}

impl SymbolicAutomaton {
    pub fn compute(grammar: &Grammar) -> Self {
        let mut automaton = SymbolicAutomaton::default();
        automaton.populate(grammar);
        automaton
    }

    fn populate(&mut self, grammar: &Grammar) {
        let first_state = LalrState::new(0, HashSet::from_iter([LalrItem::new("*".to_string())]));
    }

    fn add_state(&mut self, state: LalrState) {
        self.states.push(state);
        self.transitions.push(vec![None; self.symbols_count]);
    }
}
