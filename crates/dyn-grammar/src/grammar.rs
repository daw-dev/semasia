use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice::SliceIndex,
};

use itertools::Itertools;
use syn::Ident;

#[derive(Debug)]
pub struct Token<TokenId, Extras = ()> {
    id: TokenId,
    extras: Extras,
}

impl<TokenId: PartialEq, Extras> PartialEq for Token<TokenId, Extras> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<TokenId: Eq, Extras> Eq for Token<TokenId, Extras> {}

impl<Extras> Display for Token<usize, Extras> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id = self.id as u32;

        if id == 0 {
            return write!(f, "a");
        }

        while id > 0 {
            write!(f, "{}", char::from_u32('a' as u32 + id % 26).unwrap())?;
            id /= 26;
        }
        Ok(())
    }
}

impl<Extras> Display for Token<Ident, Extras> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl<TokenId, Extras> Token<TokenId, Extras> {
    pub fn new(id: TokenId, extras: Extras) -> Self {
        Self { id, extras }
    }

    pub fn id(&self) -> &TokenId {
        &self.id
    }

    pub fn extras(&self) -> &Extras {
        &self.extras
    }

    pub fn map<OtherTokenId, OtherExtras, F>(self, mapper: F) -> Token<OtherTokenId, OtherExtras>
    where
        F: FnOnce(TokenId, Extras) -> (OtherTokenId, OtherExtras),
    {
        let (new_id, new_extras) = mapper(self.id, self.extras);
        Token {
            id: new_id,
            extras: new_extras,
        }
    }
}

impl<TokenId> Token<TokenId> {
    pub fn with_id(id: TokenId) -> Self {
        Self { id, extras: () }
    }
}

#[derive(Debug)]
pub struct NonTerminal<NonTerminalId, Extras = ()> {
    id: NonTerminalId,
    extras: Extras,
}

impl<NonTerminalId: PartialEq, Extras> PartialEq for NonTerminal<NonTerminalId, Extras> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<NonTerminalId: Eq, Extras> Eq for NonTerminal<NonTerminalId, Extras> {}

impl<Extras> Display for NonTerminal<usize, Extras> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id = self.id as u32;

        if id == 0 {
            return write!(f, "A");
        }

        while id > 0 {
            write!(f, "{}", char::from_u32('A' as u32 + id % 26).unwrap())?;
            id /= 26;
        }
        Ok(())
    }
}

impl<Extras> Display for NonTerminal<Ident, Extras> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl<NonTerminalId, Extras> NonTerminal<NonTerminalId, Extras> {
    pub fn new(id: NonTerminalId, extras: Extras) -> Self {
        Self { id, extras }
    }

    pub fn id(&self) -> &NonTerminalId {
        &self.id
    }

    pub fn extras(&self) -> &Extras {
        &self.extras
    }

    pub fn map<OtherNonTerminalId, OtherExtras, F>(
        self,
        mapper: F,
    ) -> NonTerminal<OtherNonTerminalId, OtherExtras>
    where
        F: FnOnce(NonTerminalId, Extras) -> (OtherNonTerminalId, OtherExtras),
    {
        let (new_id, new_extras) = mapper(self.id, self.extras);
        NonTerminal {
            id: new_id,
            extras: new_extras,
        }
    }
}

impl<NonTerminalId> NonTerminal<NonTerminalId> {
    pub fn with_id(id: NonTerminalId) -> Self {
        Self { id, extras: () }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol<TokenType, NonTerminalType> {
    Token(TokenType),
    NonTerminal(NonTerminalType),
}

impl<TokenType, NonTerminalType> Display for Symbol<TokenType, NonTerminalType>
where
    TokenType: Display,
    NonTerminalType: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Token(token) => token.fmt(f),
            Symbol::NonTerminal(non_terminal) => non_terminal.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<TokenType, NonTerminalType> {
    body: Vec<Symbol<TokenType, NonTerminalType>>,
}

impl<TokenType, NonTerminalType, Idx> IndexMut<Idx> for Body<TokenType, NonTerminalType>
where
    Idx: SliceIndex<[Symbol<TokenType, NonTerminalType>]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.body.index_mut(index)
    }
}

impl<TokenType, NonTerminalType, Idx> Index<Idx> for Body<TokenType, NonTerminalType>
where
    Idx: SliceIndex<[Symbol<TokenType, NonTerminalType>]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.body.index(index)
    }
}

impl<TokenType, NonTerminalType> DerefMut for Body<TokenType, NonTerminalType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.body.deref_mut()
    }
}

impl<TokenType, NonTerminalType> Deref for Body<TokenType, NonTerminalType> {
    type Target = [Symbol<TokenType, NonTerminalType>];

    fn deref(&self) -> &Self::Target {
        self.body.deref()
    }
}

impl<TokenType, NonTerminalType> Display for Body<TokenType, NonTerminalType>
where
    TokenType: Display,
    NonTerminalType: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.body.iter().format(", "))
    }
}

pub struct Production<ProductionId, TokenType, NonTerminalType> {
    id: ProductionId,
    head: NonTerminalType,
    body: Body<TokenType, NonTerminalType>,
}

impl<ProductionId, TokenType, NonTerminalType> Display
    for Production<ProductionId, TokenType, NonTerminalType>
where
    ProductionId: Display,
    TokenType: Display,
    NonTerminalType: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} -> {}", self.id, self.head, self.body)
    }
}

impl<ProductionId: PartialEq, TokenType, NonTerminalType> PartialEq
    for Production<ProductionId, TokenType, NonTerminalType>
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<ProductionId, TokenType, NonTerminalType>
    Production<ProductionId, TokenType, NonTerminalType>
{
    pub fn id(&self) -> &ProductionId {
        &self.id
    }

    pub fn head(&self) -> &NonTerminalType {
        &self.head
    }

    pub fn body(&self) -> &Body<TokenType, NonTerminalType> {
        &self.body
    }
}

pub struct Grammar<TokenType, NonTerminalType, ProductionType, Extras> {
    tokens: Vec<TokenType>,
    non_terminals: Vec<NonTerminalType>,
    start_symbol: NonTerminalType,
    productions: Vec<ProductionType>,
    extras: Extras,
}

pub struct FirstSet<TokenId> {
    pub tokens: HashSet<TokenId>,
    pub nullable: bool,
}

impl<TokenType, NonTerminalType, ProductionType, Extras>
    Grammar<TokenType, NonTerminalType, ProductionType, Extras>
{
    pub fn tokens(&self) -> &Vec<TokenType> {
        &self.tokens
    }

    pub fn non_terminals(&self) -> &Vec<NonTerminalType> {
        &self.non_terminals
    }

    pub fn start_symbol(&self) -> &NonTerminalType {
        &self.start_symbol
    }

    pub fn productions(&self) -> &Vec<ProductionType> {
        &self.productions
    }

    pub fn extras(&self) -> &Extras {
        &self.extras
    }
}

impl<TokenId, TokenExtras, NonTerminalId, NonTerminalExtras, ProductionId, Extras>
    Grammar<
        Token<TokenId, TokenExtras>,
        NonTerminal<NonTerminalId, NonTerminalExtras>,
        Production<ProductionId, TokenId, NonTerminalId>,
        Extras,
    >
where
    TokenId: Hash + Eq,
    NonTerminalId: Hash + Eq,
    ProductionId: Hash + Eq + Clone,
{
    fn first_set_helper<'a>(
        &'a self,
        beta: &'a [Symbol<TokenId, NonTerminalId>],
        visited: &mut HashSet<&'a ProductionId>,
    ) -> FirstSet<&'a TokenId> {
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

    pub fn first_set<'a>(
        &'a self,
        beta: &'a [Symbol<TokenId, NonTerminalId>],
    ) -> FirstSet<&'a TokenId> {
        self.first_set_helper(beta, &mut HashSet::new())
    }
}
