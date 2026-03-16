use syn::Ident;

use crate::{
    EnrichedNonTerminal, EnrichedToken,
    grammar::{Grammar, NonTerminal, Production, Symbol, Token},
};
use std::collections::HashSet;

pub type SymbolicToken = Token<usize, EnrichedToken>;

pub type SymbolicNonTerminal = NonTerminal<usize, EnrichedNonTerminal>;

pub type SymbolicSymbol = Symbol<SymbolicToken, SymbolicNonTerminal>;

pub type SymbolicProduction = Production<usize, SymbolicNonTerminal, SymbolicSymbol, Ident>;

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

pub struct FirstSet<TokenId> {
    pub tokens: HashSet<TokenId>,
    pub nullable: bool,
}

impl SymbolicGrammar {
    fn first_set_helper<'a>(
        &'a self,
        beta: &'a [SymbolicSymbol],
        visited: &mut HashSet<&'a usize>,
    ) -> FirstSet<&'a SymbolicToken> {
        // eprintln!("finding firsts for ({})", beta.iter().format(", "));
        if beta.is_empty() {
            return FirstSet {
                tokens: HashSet::new(),
                nullable: true,
            };
        }

        let mut res = FirstSet {
            tokens: HashSet::new(),
            nullable: false,
        };

        for symbol in beta.iter() {
            match symbol {
                Symbol::Token(token) => {
                    // eprintln!("inserted {}", token.id());
                    res.tokens.insert(token);
                    return res;
                }
                Symbol::NonTerminal(non_terminal) => {
                    let productions = self
                        .productions()
                        .iter()
                        .filter(|prod| prod.head() == non_terminal);
                    let mut some_nullable = false;
                    for prod in productions.into_iter() {
                        // eprintln!("checking {prod}");
                        if !visited.insert(prod.id()) {
                            continue;
                        }
                        let body = prod.body();
                        let firsts = self.first_set_helper(body, visited);
                        res.tokens.extend(firsts.tokens);
                        some_nullable |= firsts.nullable;
                    }
                    if !some_nullable {
                        return res;
                    }
                }
            }
        }

        res.nullable = true;
        res
    }

    pub fn first_set<'a>(&'a self, beta: &'a [SymbolicSymbol]) -> FirstSet<&'a SymbolicToken> {
        self.first_set_helper(beta, &mut HashSet::new())
    }
}
