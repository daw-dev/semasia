use dyn_grammar::{EnrichedGrammar, lalr::LalrAutomaton, symbolic_grammar::SymbolicGrammar};
use syn::Ident;

pub struct Constructor<'a> {
    pub enriched_grammar: EnrichedGrammar,
    pub sym_grammar: SymbolicGrammar<'a>,
    pub automaton: LalrAutomaton<'a>,
    pub internal_mod_name: Option<Ident>,
}
