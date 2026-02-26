use dyn_grammar::{EnrichedGrammar, lalr::LalrAutomaton};
use std::rc::Rc;
use syn::Ident;

pub struct Constructor {
    pub enriched_grammar: Rc<EnrichedGrammar>,
    pub automaton: LalrAutomaton,
    pub internal_mod_name: Option<Ident>,
}
