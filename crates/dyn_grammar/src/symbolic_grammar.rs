use crate::non_terminal::EnrichedNonTerminal;
use crate::{EnrichedGrammar, production::EnrichedProduction};
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, rc::Rc};
use syn::Ident;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolicToken(pub usize);

impl Display for SymbolicToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id = self.0 as u32;

        if id == 0 {
            write!(f, "a")?;
        }

        while id > 0 {
            write!(f, "{}", char::from_u32('a' as u32 + id % 26).unwrap())?;
            id /= 26;
        }
        Ok(())
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolicNonTerminal(pub usize);

impl Display for SymbolicNonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id = self.0 as u32;

        if id == 0 {
            write!(f, "A")?;
        }

        while id > 0 {
            write!(f, "{}", char::from_u32('A' as u32 + id % 26).unwrap())?;
            id /= 26;
        }
        Ok(())
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum SymbolicSymbol {
    Token(SymbolicToken),
    NonTerminal(SymbolicNonTerminal),
}

impl Display for SymbolicSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolicSymbol::Token(tok) => write!(f, "{tok}"),
            SymbolicSymbol::NonTerminal(nt) => write!(f, "{nt}"),
        }
    }
}

#[derive(Debug)]
pub struct SymbolicProduction {
    production_id: usize,
    head: SymbolicNonTerminal,
    body: Vec<SymbolicSymbol>,
}

impl Display for SymbolicProduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} -> ({})",
            self.production_id,
            self.head(),
            self.body().iter().format(", ")
        )
    }
}

impl SymbolicProduction {
    pub fn id(&self) -> usize {
        self.production_id
    }

    pub fn head(&self) -> &SymbolicNonTerminal {
        &self.head
    }

    pub fn body(&self) -> &Vec<SymbolicSymbol> {
        &self.body
    }

    pub fn arity(&self) -> usize {
        self.body.len()
    }

    pub fn special_production(start_symbol: SymbolicNonTerminal) -> Self {
        Self {
            production_id: usize::MAX,
            head: SymbolicNonTerminal(usize::MAX),
            body: vec![SymbolicSymbol::NonTerminal(start_symbol)],
        }
    }
}

pub struct SymbolicFirstSet {
    pub tokens: HashSet<SymbolicToken>,
    pub nullable: bool,
}

pub struct SymbolicFollowSet {
    pub tokens: HashSet<SymbolicToken>,
    pub eof_follows: bool,
}

#[derive(Debug)]
pub struct SymbolicGrammar {
    enriched_grammar: Option<Rc<EnrichedGrammar>>,
    token_count: usize,
    non_terminal_count: usize,
    start_symbol: SymbolicNonTerminal,
    special_production: SymbolicProduction,
    productions: Vec<SymbolicProduction>,
}

impl Display for SymbolicGrammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SymbolicGrammar {{ ")?;
        write!(
            f,
            "non_terminals: [{}], ",
            (0..self.non_terminal_count)
                .map(SymbolicNonTerminal)
                .format(", ")
        )?;
        write!(
            f,
            "tokens: [{}], ",
            (0..self.token_count).map(SymbolicToken).format(", ")
        )?;
        write!(f, "start_symbol: {}, ", self.start_symbol,)?;
        write!(
            f,
            "productions: [{}] }}",
            self.productions.iter().format(", ")
        )
    }
}

impl SymbolicGrammar {
    pub fn enriched_grammar(&self) -> &EnrichedGrammar {
        &self.enriched_grammar.as_ref().unwrap()
    }

    pub fn get_production(&self, id: usize) -> Option<&SymbolicProduction> {
        if id == usize::MAX {
            return Some(&self.special_production);
        }
        self.productions.get(id)
    }

    pub fn get_productions_with_head(&self, head: SymbolicNonTerminal) -> Vec<&SymbolicProduction> {
        self.productions
            .iter()
            .filter(|prod| prod.head == head)
            .collect()
    }

    pub fn token_count(&self) -> usize {
        self.token_count
    }

    pub fn non_terminal_count(&self) -> usize {
        self.non_terminal_count
    }

    fn find_symbol(enriched_grammar: &EnrichedGrammar, ident: &Ident) -> Option<SymbolicSymbol> {
        enriched_grammar
            .token_id(ident)
            .map(SymbolicToken)
            .map(SymbolicSymbol::Token)
            .or_else(|| {
                enriched_grammar
                    .non_terminal_id(ident)
                    .map(SymbolicNonTerminal)
                    .map(SymbolicSymbol::NonTerminal)
            })
    }

    fn map_production(
        enriched_grammar: &EnrichedGrammar,
        id: usize,
        enriched_production: &EnrichedProduction,
    ) -> SymbolicProduction {
        SymbolicProduction {
            production_id: id,
            head: SymbolicNonTerminal(
                enriched_grammar
                    .non_terminal_id(enriched_production.head())
                    .unwrap(),
            ),
            body: enriched_production
                .body()
                .iter()
                .map(|sym| match sym {
                    crate::enriched_symbol::EnrichedSymbol::Token(enriched_token) => {
                        SymbolicSymbol::Token(SymbolicToken(
                            enriched_grammar.token_id(enriched_token.ident()).unwrap(),
                        ))
                    }
                    crate::enriched_symbol::EnrichedSymbol::NonTerminal(enriched_non_terminal) => {
                        SymbolicSymbol::NonTerminal(SymbolicNonTerminal(
                            enriched_grammar
                                .non_terminal_id(enriched_non_terminal.ident())
                                .unwrap(),
                        ))
                    }
                })
                .collect(),
        }
    }

    fn first_set_helper(
        &self,
        beta: &[SymbolicSymbol],
        visited: &mut HashSet<usize>,
    ) -> SymbolicFirstSet {
        eprintln!("finding firsts for ({})", beta.iter().format(", "));
        if beta.is_empty() {
            return SymbolicFirstSet {
                tokens: HashSet::new(),
                nullable: true,
            };
        }

        let mut res = SymbolicFirstSet {
            tokens: HashSet::new(),
            nullable: false,
        };

        for symbol in beta.iter() {
            match symbol {
                SymbolicSymbol::Token(token) => {
                    eprintln!("inserted {token}");
                    res.tokens.insert(*token);
                    return res;
                }
                SymbolicSymbol::NonTerminal(non_terminal) => {
                    let productions = self.get_productions_with_head(*non_terminal);
                    for prod in productions.into_iter() {
                        if !visited.insert(prod.id()) {
                            continue;
                        }
                        let body = prod.body();
                        let firsts = self.first_set_helper(body, visited);
                        res.tokens.extend(firsts.tokens);
                        if !firsts.nullable {
                            return res;
                        }
                    }
                }
            }
        }

        res.nullable = true;
        res
    }

    pub fn first_set(&self, beta: &[SymbolicSymbol]) -> SymbolicFirstSet {
        self.first_set_helper(beta, &mut HashSet::new())
    }
}

impl From<Rc<EnrichedGrammar>> for SymbolicGrammar {
    fn from(value: Rc<EnrichedGrammar>) -> Self {
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
            enriched_grammar: Some(value),
            token_count,
            non_terminal_count,
            start_symbol,
            special_production: SymbolicProduction::special_production(start_symbol),
            productions,
        }
    }
}

#[test]
fn firsts_test() {
    let grammar = SymbolicGrammar {
        enriched_grammar: None,
        token_count: 4,
        non_terminal_count: 4,
        start_symbol: SymbolicNonTerminal(0),
        special_production: SymbolicProduction::special_production(SymbolicNonTerminal(0)),
        productions: vec![
            SymbolicProduction {
                production_id: 0,
                head: SymbolicNonTerminal(0),
                body: [1, 2, 3]
                    .into_iter()
                    .map(SymbolicNonTerminal)
                    .map(SymbolicSymbol::NonTerminal)
                    .collect(),
            },
            SymbolicProduction {
                production_id: 1,
                head: SymbolicNonTerminal(1),
                body: vec![
                    SymbolicSymbol::NonTerminal(SymbolicNonTerminal(1)),
                    SymbolicSymbol::Token(SymbolicToken(0)),
                ],
            },
            SymbolicProduction {
                production_id: 2,
                head: SymbolicNonTerminal(1),
                body: Vec::new(),
            },
            SymbolicProduction {
                production_id: 3,
                head: SymbolicNonTerminal(2),
                body: vec![SymbolicSymbol::Token(SymbolicToken(1))],
            },
            SymbolicProduction {
                production_id: 4,
                head: SymbolicNonTerminal(2),
                body: Vec::new(),
            },
            SymbolicProduction {
                production_id: 5,
                head: SymbolicNonTerminal(3),
                body: vec![SymbolicSymbol::Token(SymbolicToken(2))],
            },
            SymbolicProduction {
                production_id: 6,
                head: SymbolicNonTerminal(3),
                body: vec![SymbolicSymbol::Token(SymbolicToken(3))],
            },
        ],
    };
    println!("{grammar}");

    let firsts = grammar.first_set(
        &[2, 3]
            .map(SymbolicNonTerminal)
            .map(SymbolicSymbol::NonTerminal),
    );
    println!("{:?}", firsts.tokens);
    println!("{}", firsts.nullable);
}
