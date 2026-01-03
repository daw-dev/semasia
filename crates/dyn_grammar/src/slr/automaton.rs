use crate::{
    slr::item::SlrItem,
    symbolic_grammar::{SymbolicGrammar, SymbolicNonTerminal, SymbolicSymbol, SymbolicToken},
};
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, usize};

#[derive(Debug)]
pub struct SlrState {
    pub(crate) kernel: HashSet<SlrItem>,
    pub(crate) marked: bool,
}

impl PartialEq for SlrState {
    fn eq(&self, other: &Self) -> bool {
        self.kernel == other.kernel
    }
}

impl SlrState {
    pub fn new(kernel: HashSet<SlrItem>) -> Self {
        Self {
            kernel,
            marked: false,
        }
    }

    fn closure(&self, grammar: &SymbolicGrammar) -> HashSet<SlrItem> {
        let mut stack = self.kernel.iter().cloned().collect_vec();
        let mut res = self.kernel.clone();
        while let Some(item) = stack.pop() {
            let SymbolicSymbol::NonTerminal(non_terminal) = item.pointed_symbol(grammar) else {
                continue;
            };

            for new_item in grammar
                .get_productions_with_head(non_terminal)
                .into_iter()
                .map(|prod| SlrItem::new(prod.id()))
            {
                if res.contains(&new_item) {
                    continue;
                }
                stack.push(new_item.clone());
                res.insert(new_item);
            }
        }
        res
    }
}

#[derive(Debug)]
pub struct TransitionTables {
    token_table: Vec<Vec<Option<SymbolicToken>>>,
    non_terminal_table: Vec<Vec<Option<SymbolicNonTerminal>>>,
}

impl TransitionTables {
    pub fn new() -> Self {
        Self {
            token_table: Vec::new(),
            non_terminal_table: Vec::new(),
        }
    }

    fn add_transitions(
        &mut self,
        token_transitions: Vec<Option<SymbolicToken>>,
        non_terminal_transitions: Vec<Option<SymbolicNonTerminal>>,
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
        writeln!(f, "")?;
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
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SlrAutomaton<'a> {
    pub(crate) grammar: &'a SymbolicGrammar,
    pub(crate) states: Vec<SlrState>,
    pub(crate) transitions: TransitionTables,
}

impl<'a> SlrAutomaton<'a> {
    pub fn compute(grammar: &'a SymbolicGrammar) -> Self {
        let mut automaton = SlrAutomaton {
            grammar,
            states: Vec::new(),
            transitions: TransitionTables::new(),
        };
        automaton.populate();
        automaton
    }

    fn populate(&mut self) {
        let first_state = SlrState::new(HashSet::from_iter([SlrItem::new(usize::MAX)]));
        self.add_state(first_state);

        while let Some(state) = self.states.iter_mut().find(|state| !state.marked) {
            state.marked = true;
            let closure = state.closure(self.grammar);
            let mut token_transitions = vec![HashSet::new(); self.grammar.token_count()];
            let mut non_terminal_transitions =
                vec![HashSet::new(); self.grammar.non_terminal_count()];
            for (symbol, mut item) in closure.into_iter().filter_map(|item| {
                (!item.is_final_item(self.grammar))
                    .then(|| (item.pointed_symbol(self.grammar), item))
            }) {
                item.move_marker();
                match symbol {
                    SymbolicSymbol::Token(tok) => {
                        token_transitions[tok].insert(item);
                    }
                    SymbolicSymbol::NonTerminal(nt) => {
                        non_terminal_transitions[nt].insert(item);
                    }
                    SymbolicSymbol::EOF => unreachable!(),
                }
            }
            let token_transitions = token_transitions
                .into_iter()
                .map(|kernel| (!kernel.is_empty()).then(|| SlrState::new(kernel)))
                .collect::<Vec<_>>();
            let non_terminal_transitions = non_terminal_transitions
                .into_iter()
                .map(|kernel| (!kernel.is_empty()).then(|| SlrState::new(kernel)))
                .collect::<Vec<_>>();
            let token_transitions = token_transitions
                .into_iter()
                .map(|target_state| {
                    target_state.map(|target_state| {
                        match self.states.iter().position(|state| state == &target_state) {
                            Some(i) => i,
                            None => {
                                let state_id = self.states.len();
                                self.add_state(target_state);
                                state_id
                            }
                        }
                    })
                })
                .collect::<Vec<_>>();
            let non_terminal_transitions = non_terminal_transitions
                .into_iter()
                .map(|target_state| {
                    target_state.map(|target_state| {
                        match self.states.iter().position(|state| state == &target_state) {
                            Some(i) => i,
                            None => {
                                let state_id = self.states.len();
                                self.add_state(target_state);
                                state_id
                            }
                        }
                    })
                })
                .collect::<Vec<_>>();
            self.transitions
                .add_transitions(token_transitions, non_terminal_transitions);
        }
    }

    fn add_state(&mut self, state: SlrState) {
        self.states.push(state);
    }
}

impl Display for SlrAutomaton<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SlrAutomaton:")?;
        writeln!(f, "States:")?;
        for (state_id, state) in self.states.iter().enumerate() {
            writeln!(
                f,
                "{state_id}: {{{}}}",
                state
                    .kernel
                    .iter()
                    .map(|item| {
                        let production = self.grammar.get_production(item.production_id).unwrap();
                        let (before_marker, after_marker) =
                            production.body().split_at(item.marker_position);
                        format!(
                            "{} -> {}·{}",
                            production.head(),
                            before_marker.iter().format(" "),
                            after_marker.iter().format(" ")
                        )
                    })
                    .format(", ")
            )?;
        }
        write!(f, "{}", self.transitions)
    }
}
