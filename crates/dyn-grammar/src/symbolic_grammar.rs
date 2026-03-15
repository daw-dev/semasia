use crate::{EnrichedGrammar, grammar::{Grammar, NonTerminal, Production, Symbol, Token}};
use std::collections::HashSet;

pub type SymbolicToken = Token<usize>;

pub type SymbolicNonTerminal = NonTerminal<usize>;

pub type SymbolicSymbol = Symbol<SymbolicToken, SymbolicNonTerminal>;

pub type SymbolicProduction = Production<usize, SymbolicToken, SymbolicNonTerminal>;

// impl SymbolicProduction {
//     pub fn special_production(start_symbol: SymbolicNonTerminal) -> Self {
//         Self {
//             production_id: usize::MAX,
//             head: SymbolicNonTerminal(usize::MAX),
//             body: vec![SymbolicSymbol::NonTerminal(start_symbol)],
//         }
//     }
// }


pub struct SymbolicFirstSet {
    pub tokens: HashSet<SymbolicToken>,
    pub nullable: bool,
}

pub struct SymbolicFollowSet {
    pub tokens: HashSet<SymbolicToken>,
    pub eof_follows: bool,
}

pub type SymbolicGrammar = Grammar<SymbolicToken, SymbolicNonTerminal, SymbolicProduction>;

// #[derive(Debug)]
// pub struct SymbolicGrammar {
//     enriched_grammar: Option<Rc<EnrichedGrammar>>,
//     token_count: usize,
//     non_terminal_count: usize,
//     start_symbol: SymbolicNonTerminal,
//     special_production: SymbolicProduction,
//     productions: Vec<SymbolicProduction>,
// }

impl SymbolicGrammar {
    // pub fn enriched_grammar(&self) -> &EnrichedGrammar {
    //     self.extras()
    // }

    pub fn get_production(&self, id: usize) -> Option<&SymbolicProduction> {
        if id == usize::MAX {
            return Some(&self.special_production);
        }
        self.productions().get(id)
    }

    // fn map_production(
    //     enriched_grammar: &EnrichedGrammar,
    //     id: usize,
    //     enriched_production: &EnrichedProduction,
    // ) -> SymbolicProduction {
    //     SymbolicProduction {
    //         production_id: id,
    //         head: SymbolicNonTerminal(
    //             enriched_grammar
    //                 .non_terminal_id(enriched_production.head())
    //                 .unwrap(),
    //         ),
    //         body: enriched_production
    //             .body()
    //             .iter()
    //             .map(|sym| match sym {
    //                 crate::enriched_symbol::EnrichedSymbol::Token(enriched_token) => {
    //                     SymbolicSymbol::Token(SymbolicToken(
    //                         enriched_grammar.token_id(enriched_token.ident()).unwrap(),
    //                     ))
    //                 }
    //                 crate::enriched_symbol::EnrichedSymbol::NonTerminal(enriched_non_terminal) => {
    //                     SymbolicSymbol::NonTerminal(SymbolicNonTerminal(
    //                         enriched_grammar
    //                             .non_terminal_id(enriched_non_terminal.ident())
    //                             .unwrap(),
    //                     ))
    //                 }
    //             })
    //             .collect(),
    //     }
    // }
}

impl From<EnrichedGrammar> for SymbolicGrammar {
    fn from(value: EnrichedGrammar) -> Self {
        let token_count = value.tokens().len();
        let non_terminal_count = value.non_terminals().len();
        let start_symbol =
            SymbolicNonTerminal(value.non_terminal_id(value.start_symbol().ident()).unwrap());
        let productions = value
            .productions()
            .iter()
            .enumerate()
            .map(|(id, prod)| SymbolicGrammar::map_production(&value, id, prod))
            .collect();
        Self {
            start_symbol,
            special_production: SymbolicProduction::special_production(start_symbol),
            productions,
        }
    }
}

// #[test]
// fn firsts_test() {
//     let grammar = SymbolicGrammar {
//         enriched_grammar: None,
//         token_count: 4,
//         non_terminal_count: 4,
//         start_symbol: SymbolicNonTerminal(0),
//         special_production: SymbolicProduction::special_production(SymbolicNonTerminal(0)),
//         productions: vec![
//             SymbolicProduction {
//                 production_id: 0,
//                 head: SymbolicNonTerminal(0),
//                 body: [1, 2, 3]
//                     .into_iter()
//                     .map(SymbolicNonTerminal)
//                     .map(SymbolicSymbol::NonTerminal)
//                     .collect(),
//             },
//             SymbolicProduction {
//                 production_id: 1,
//                 head: SymbolicNonTerminal(1),
//                 body: vec![
//                     SymbolicSymbol::NonTerminal(SymbolicNonTerminal(1)),
//                     SymbolicSymbol::Token(SymbolicToken(0)),
//                 ],
//             },
//             SymbolicProduction {
//                 production_id: 2,
//                 head: SymbolicNonTerminal(1),
//                 body: Vec::new(),
//             },
//             SymbolicProduction {
//                 production_id: 3,
//                 head: SymbolicNonTerminal(2),
//                 body: vec![SymbolicSymbol::Token(SymbolicToken(1))],
//             },
//             SymbolicProduction {
//                 production_id: 4,
//                 head: SymbolicNonTerminal(2),
//                 body: Vec::new(),
//             },
//             SymbolicProduction {
//                 production_id: 5,
//                 head: SymbolicNonTerminal(3),
//                 body: vec![SymbolicSymbol::Token(SymbolicToken(2))],
//             },
//             SymbolicProduction {
//                 production_id: 6,
//                 head: SymbolicNonTerminal(3),
//                 body: vec![SymbolicSymbol::Token(SymbolicToken(3))],
//             },
//         ],
//     };
//     println!("{grammar}");
//
//     let firsts = grammar.first_set(
//         &[2, 3]
//             .map(SymbolicNonTerminal)
//             .map(SymbolicSymbol::NonTerminal),
//     );
//     println!("{{{}}}", firsts.tokens.iter().format(", "));
//     println!("{}", firsts.nullable);
// }
