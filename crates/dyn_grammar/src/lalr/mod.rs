use std::{
    cell::RefCell, collections::{HashMap, HashSet}, rc::Rc}
;

use itertools::Itertools;

use crate::{
    slr::{
        automaton::{SlrAutomaton, SlrState, TransitionTables},
        item::SlrItem,
    },
    symbolic_grammar::{SymbolicGrammar, SymbolicSymbol, SymbolicToken},
};

struct LalrItem {
    production_id: usize,
    marker_position: usize,
    natural_lookahead: HashSet<SymbolicToken>,
    dependencies: Vec<(Rc<RefCell<LalrItem>>, HashSet<SymbolicToken>)>,
}

struct LalrState {
    kernel: HashMap<SlrItem, HashSet<SymbolicToken>>,
    marked: bool,
}

impl PartialEq for LalrState {
    fn eq(&self, other: &Self) -> bool {
        self.kernel == other.kernel
    }
}

impl From<SlrState> for LalrState {
    fn from(value: SlrState) -> Self {
        Self {
            kernel: value
                .kernel
                .into_iter()
                .map(Into::into)
                .zip(std::iter::repeat_with(|| HashSet::new()))
                .collect(),
            marked: value.marked,
        }
    }
}

impl LalrState {
    fn closure(&self, grammar: &SymbolicGrammar) -> HashMap<SlrItem, HashSet<SymbolicToken>> {
        let mut stack = self.kernel.clone().into_iter().collect_vec();
        let mut res = self.kernel.clone();
        while let Some((item, lookahead_set)) = stack.pop() {
            let SymbolicSymbol::NonTerminal(non_terminal) = item.pointed_symbol(grammar) else {
                continue;
            };

            for new_item in grammar
                .get_productions_with_head(non_terminal)
                .into_iter()
                .map(|prod| SlrItem::new(prod.id()))
            {
                match res.get_mut(&new_item) {
                    Some(item) => {
                        item.insert(todo!());
                    }
                    None => {
                        stack.push((new_item.clone(), lookahead_set.clone()));
                        res.insert(new_item, todo!());
                    }
                }
            }
        }
        res
    }
}

struct LalrAutomaton<'a> {
    grammar: &'a SymbolicGrammar,
    states: Vec<LalrState>,
    transitions: TransitionTables,
}

impl<'a> LalrAutomaton<'a> {
    fn find_lookaheads(&mut self) {
        let start_state = &mut self.states[0];
        for item in start_state.kernel.iter_mut() {}
    }
}

impl<'a> From<SlrAutomaton<'a>> for LalrAutomaton<'a> {
    fn from(value: SlrAutomaton<'a>) -> Self {
        let SlrAutomaton {
            grammar,
            states: slr_states,
            transitions,
        } = value;
        let states = slr_states.into_iter().map(Into::into).collect();

        let mut res = Self {
            grammar,
            states,
            transitions,
        };

        res.find_lookaheads();

        res
    }
}
