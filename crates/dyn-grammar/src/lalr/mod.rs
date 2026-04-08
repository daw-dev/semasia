#![allow(clippy::mutable_key_type)]

use crate::{
    conflicts::{Associativity, ProductionPriority},
    parsing::{
        action::{EofAction, TokenAction},
        tables::{EofTable, NonTerminalTable, TokenTable, TransitionTables},
    },
    symbolic_grammar::{SymbolicGrammar, SymbolicProduction, SymbolicSymbol, SymbolicToken},
};
use itertools::Itertools;
use proc_macro_error::{abort_if_dirty, emit_error};
use std::{cell::RefCell, cmp::Ordering, collections::HashSet, fmt::Display, hash::Hash, rc::Rc};

#[derive(Clone)]
struct LookAhead<'a> {
    tokens: HashSet<&'a SymbolicToken>,
    can_eof_follow: bool,
}

impl Display for LookAhead<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tokens = self.tokens.iter().collect_vec();
        tokens.sort();
        write!(
            f,
            "{{{}}}",
            tokens
                .into_iter()
                .map(|tok| format!("{tok}"))
                .chain(self.can_eof_follow.then_some("$".to_string()))
                .format(", ")
        )
    }
}

#[derive(Clone)]
struct LookAheadNodeRef<'a>(Rc<RefCell<LookAheadNode<'a>>>);

impl<'a> LookAheadNodeRef<'a> {
    pub fn initial_lookahead_node(counter: &mut usize) -> LookAheadNodeRef<'a> {
        Self::new(
            counter,
            LookAhead {
                tokens: HashSet::new(),
                can_eof_follow: true,
            },
            Vec::new(),
        )
    }

    pub fn new(
        counter: &mut usize,
        natural_lookahead: LookAhead<'a>,
        dependencies: Vec<LookAheadNodeRef<'a>>,
    ) -> Self {
        let node_id = *counter;
        *counter += 1;
        Self(Rc::new(RefCell::new(LookAheadNode {
            node_id,
            natural_lookahead,
            dependencies,
        })))
    }

    fn compute_lookahead_helper(&self, visited: &mut HashSet<usize>) -> LookAhead<'a> {
        // TODO: not so simple, graph could have cycles
        let borrow = self.0.borrow();
        let mut res = borrow.natural_lookahead.clone();
        if visited.contains(&borrow.node_id) {
            return res;
        }
        visited.insert(borrow.node_id);
        for dep in borrow.dependencies.iter() {
            let lh = dep.compute_lookahead_helper(visited);
            res.tokens.extend(lh.tokens);
            res.can_eof_follow |= lh.can_eof_follow;
        }
        res
    }

    pub fn compute_lookahead(&self) -> LookAhead<'a> {
        self.compute_lookahead_helper(&mut HashSet::new())
    }

    pub fn add_dependency(&self, dependency: LookAheadNodeRef<'a>) {
        self.0.borrow_mut().dependencies.push(dependency);
    }
}

struct LookAheadNode<'a> {
    node_id: usize,
    natural_lookahead: LookAhead<'a>,
    dependencies: Vec<LookAheadNodeRef<'a>>,
}

#[derive(Clone)]
struct LalrItem<'a> {
    production: &'a SymbolicProduction,
    marker_position: usize,
    lookahead_node: LookAheadNodeRef<'a>,
    is_accepting: bool,
}

impl Hash for LalrItem<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.production.hash(state);
        self.marker_position.hash(state);
    }
}

impl PartialEq for LalrItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.production.id() == other.production.id()
            && self.marker_position == other.marker_position
    }
}

impl Eq for LalrItem<'_> {}

impl Display for LalrItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (before_marker, after_marker) = self.production.body().split_at(self.marker_position);
        write!(
            f,
            "{}: {} -> ({}·{})",
            self.production.extras().0,
            self.production.head(),
            before_marker.iter().format(", "),
            after_marker.iter().format(", ")
        )
    }
}

impl<'a> LalrItem<'a> {
    pub fn new(production: &'a SymbolicProduction, lookahead_node: LookAheadNodeRef<'a>) -> Self {
        Self {
            production,
            marker_position: 0,
            lookahead_node,
            is_accepting: false,
        }
    }

    pub fn accepting(production: &'a SymbolicProduction, counter: &mut usize) -> Self {
        Self {
            production,
            marker_position: 0,
            lookahead_node: LookAheadNodeRef::initial_lookahead_node(counter),
            is_accepting: true,
        }
    }

    pub fn pointed_symbol(&self, grammar: &'a SymbolicGrammar) -> Option<&'a SymbolicSymbol> {
        grammar
            .productions()
            .get(*self.production.id())
            .expect("production not found")
            .body()
            .get(self.marker_position)
    }

    pub fn move_marker(&mut self) {
        self.marker_position += 1;
    }

    fn is_reducing(&self, grammar: &SymbolicGrammar) -> bool {
        self.marker_position
            == grammar
                .productions()
                .get(*self.production.id())
                .unwrap()
                .arity()
    }

    fn is_accepting(&self) -> bool {
        self.is_accepting
    }
}

struct LalrState<'a> {
    kernel: HashSet<LalrItem<'a>>,
    marked: bool,
    epsilon_items: HashSet<LalrItem<'a>>,
}

impl PartialEq for LalrState<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kernel == other.kernel
    }
}

impl Display for LalrState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LalrState")
            .field(
                "kernel",
                &format!("{{{}}}", self.kernel.iter().format(", ")),
            )
            .finish_non_exhaustive()
    }
}

impl<'a> LalrState<'a> {
    fn new(kernel: HashSet<LalrItem<'a>>) -> Self {
        Self {
            kernel,
            marked: false,
            epsilon_items: HashSet::new(),
        }
    }

    fn closure(&self, counter: &mut usize, grammar: &'a SymbolicGrammar) -> HashSet<LalrItem<'a>> {
        let mut stack = self.kernel.clone().into_iter().collect_vec();
        let mut res = self.kernel.clone();

        while let Some(item) = stack.pop() {
            if item.is_reducing(grammar) {
                continue;
            }

            let Some(SymbolicSymbol::NonTerminal(non_terminal)) = item.pointed_symbol(grammar)
            else {
                continue;
            };

            let item_production = item.production;

            let beta = &item_production.body()[item.marker_position + 1..];

            let firsts = grammar.first_set(beta);
            let natural_lookahead = LookAhead {
                tokens: firsts.tokens,
                can_eof_follow: false,
            };
            let dependencies = if firsts.nullable {
                vec![item.lookahead_node]
            } else {
                Vec::new()
            };
            let lookahead_node = LookAheadNodeRef::new(counter, natural_lookahead, dependencies);

            for new_item in grammar
                .productions()
                .iter()
                .filter(|prod| prod.head() == non_terminal)
                .map(|prod| LalrItem::new(prod, lookahead_node.clone()))
            {
                match res.get(&new_item) {
                    Some(item) => {
                        item.lookahead_node.add_dependency(lookahead_node.clone());
                    }
                    None => {
                        stack.push(new_item.clone());
                        res.insert(new_item);
                    }
                }
            }
        }
        res
    }
}

pub struct LalrAutomaton<'a> {
    grammar: &'a SymbolicGrammar,
    states: Vec<LalrState<'a>>,
    transitions: TransitionTables,
}

impl<'a> LalrAutomaton<'a> {
    pub fn compute(grammar: &'a SymbolicGrammar) -> Self {
        let mut automaton = Self {
            grammar,
            states: Vec::new(),
            transitions: TransitionTables::new(),
        };
        automaton.populate();
        automaton
    }

    pub fn populate(&mut self) {
        let mut counter = 0;
        let first_state = LalrState::new(HashSet::from_iter([LalrItem::accepting(
            self.grammar.productions().last().unwrap(),
            &mut counter,
        )]));
        self.states.push(first_state);

        while let Some(state) = self.states.iter_mut().find(|state| !state.marked) {
            state.marked = true;
            let closure = state.closure(&mut counter, self.grammar);
            for eps_item in closure.iter().filter(|item| {
                self.grammar
                    .productions()
                    .get(*item.production.id())
                    .unwrap()
                    .arity()
                    == 0
            }) {
                state.epsilon_items.insert(eps_item.clone());
            }
            let mut token_transitions = vec![HashSet::new(); self.grammar.token_count()];
            let mut non_terminal_transitions =
                vec![HashSet::new(); self.grammar.non_terminal_count()];
            for (symbol, mut item) in closure.into_iter().filter_map(|item| {
                (!item.is_reducing(self.grammar))
                    .then(|| (item.pointed_symbol(self.grammar).unwrap(), item))
            }) {
                item.move_marker();
                match symbol {
                    SymbolicSymbol::Token(tok) => {
                        token_transitions[*tok.id()].insert(item);
                    }
                    SymbolicSymbol::NonTerminal(nt) => {
                        non_terminal_transitions[*nt.id()].insert(item);
                    }
                }
            }
            let token_transitions = token_transitions
                .into_iter()
                .map(|kernel| (!kernel.is_empty()).then(|| LalrState::new(kernel)))
                .collect::<Vec<_>>();
            let non_terminal_transitions = non_terminal_transitions
                .into_iter()
                .map(|kernel| (!kernel.is_empty()).then(|| LalrState::new(kernel)))
                .collect::<Vec<_>>();
            let token_transitions = token_transitions
                .into_iter()
                .map(|target_state| {
                    target_state.map(|target_state| {
                        match self.states.iter().position(|state| state == &target_state) {
                            Some(i) => {
                                for new_item in target_state.kernel.iter() {
                                    self.states[i]
                                        .kernel
                                        .get(new_item)
                                        .unwrap()
                                        .lookahead_node
                                        .add_dependency(new_item.lookahead_node.clone());
                                }
                                i
                            }
                            None => {
                                let state_id = self.states.len();
                                self.states.push(target_state);
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
                                self.states.push(target_state);
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

    pub fn states_count(&self) -> usize {
        self.states.len()
    }

    pub fn generate_tables(&self) -> (TokenTable, EofTable, NonTerminalTable) {
        let mut token_table = TokenTable::new(self.grammar.token_count());
        let mut eof_table = EofTable::new();
        let mut goto_table = NonTerminalTable::new(self.grammar.non_terminal_count());

        for ((state_id, state), (token_transitions, non_terminal_transitions)) in
            self.states.iter().enumerate().zip(self.transitions.iter())
        {
            token_table.add_state();
            eof_table.add_state();

            for reducing_item in state
                .kernel
                .iter()
                .filter(|item| item.is_reducing(self.grammar))
                .chain(state.epsilon_items.iter())
            {
                let lookahead = reducing_item.lookahead_node.compute_lookahead();
                for token in lookahead.tokens.into_iter() {
                    let production_id = *reducing_item.production.id();
                    let mut action = TokenAction::Reduce(production_id);
                    let entry = &mut token_table[(state_id, *token.id())];
                    if let Some(TokenAction::Reduce(reduce)) = entry.take() {
                        let old_reduce = &self.grammar().productions()[reduce];
                        let new_reduce = &self.grammar().productions()[production_id];
                        match old_reduce.extras().1.cmp(&new_reduce.extras().1) {
                            Ordering::Less => {
                                action = TokenAction::Reduce(reduce);
                            }
                            Ordering::Equal => {
                                emit_error!(
                                    old_reduce.extras().0, "reduce/reduce conflict";
                                    note = old_reduce.extras().0.span() => "put #[priority(<value>)]"
                                );
                            }
                            Ordering::Greater => {}
                        }
                    }
                    *entry = Some(action);
                }
                if lookahead.can_eof_follow {
                    let production_id = *reducing_item.production.id();
                    let mut action = if reducing_item.is_accepting() {
                        EofAction::Accept
                    } else {
                        EofAction::Reduce(production_id)
                    };
                    let entry = &mut eof_table[state_id];
                    if let Some(EofAction::Reduce(reduce)) = entry.take() {
                        let old_reduce = &self.grammar().productions()[reduce];
                        let new_reduce = &self.grammar().productions()[production_id];
                        match old_reduce.extras().1.cmp(&new_reduce.extras().1) {
                            Ordering::Less => {
                                action = EofAction::Reduce(reduce);
                            }
                            Ordering::Equal => {
                                emit_error!(
                                    old_reduce.extras().0, "reduce/reduce conflict";
                                    note = old_reduce.extras().0.span() => "put #[priority(<value>)]"
                                );
                            }
                            Ordering::Greater => {}
                        }
                    }
                    *entry = Some(action);
                }
            }

            for (token_id, target) in token_transitions.iter().enumerate() {
                let Some(target) = target else {
                    continue;
                };
                let token = &self.grammar().tokens()[token_id];
                let mut action = TokenAction::Shift(*target);
                let entry = &mut token_table[(state_id, token_id)];
                if let Some(TokenAction::Reduce(reduce)) = entry.take() {
                    let reduce_production = &self.grammar().productions()[reduce];
                    let prod_priority = reduce_production.extras().1;
                    let token_priority = token.extras().extras().1;
                    let ord = match (prod_priority, token_priority) {
                        (ProductionPriority::None, None) => Ordering::Equal,
                        (ProductionPriority::Inherited(_), None) => Ordering::Greater,
                        (ProductionPriority::Explicit(_), None) => Ordering::Greater,
                        (ProductionPriority::None, Some(_)) => Ordering::Less,
                        (ProductionPriority::Inherited(a), Some(b)) => a.cmp(&b),
                        (ProductionPriority::Explicit(a), Some(b)) => a.cmp(&b),
                    };
                    match ord {
                        Ordering::Less => {}
                        Ordering::Equal => match token.extras().extras().2 {
                            Associativity::Unspecified => {
                                let production = &self.grammar.productions()[reduce];
                                emit_error!(
                                    reduce_production.extras().0,
                                    "shift/reduce conflict (priority: {:?})",
                                    reduce_production.extras().1;
                                    note = token.extras().id().span() => "this token has the same priority";
                                    note = "what happens when after seeing {} you see {}?", production, token;
                                );
                            }
                            Associativity::Left => {
                                action = TokenAction::Reduce(reduce);
                            }
                            Associativity::Right => {}
                        },
                        Ordering::Greater => action = TokenAction::Reduce(reduce),
                    }
                }
                *entry = Some(action);
            }

            goto_table.add_state();
            for (non_terminal, target) in non_terminal_transitions.iter().enumerate() {
                goto_table[(state_id, non_terminal)] = *target;
            }
        }

        abort_if_dirty();

        (token_table, eof_table, goto_table)
    }

    pub fn grammar(&self) -> &SymbolicGrammar {
        self.grammar
    }
}

impl<'a> From<&'a SymbolicGrammar> for LalrAutomaton<'a> {
    fn from(value: &'a SymbolicGrammar) -> Self {
        Self::compute(value)
    }
}

impl Display for LalrAutomaton<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LalrAutomaton")
            .field(
                "states",
                &format!(
                    "[{}]",
                    self.states
                        .iter()
                        .enumerate()
                        .map(|(id, state)| { format!("{}: {}", id, state) })
                        .format(", ")
                ),
            )
            .field("transitions", &self.transitions)
            .finish()
    }
}
