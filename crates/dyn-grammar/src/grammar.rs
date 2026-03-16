use itertools::Itertools;
use std::{
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice::SliceIndex,
};

use crate::{
    EnrichedGrammar, EnrichedNonTerminal, EnrichedSymbol, EnrichedToken, production::{EnrichedBaseProduction, EnrichedProduction}, symbolic_grammar::{
        SymbolicGrammar, SymbolicNonTerminal, SymbolicProduction, SymbolicSymbol, SymbolicToken,
    }
};

#[derive(Debug, Clone, Copy)]
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

impl<TokenId: Hash, Extras> Hash for Token<TokenId, Extras> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<TokenId: PartialOrd, Extras> PartialOrd for Token<TokenId, Extras> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<TokenId: Ord, Extras> Ord for Token<TokenId, Extras> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
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

#[derive(Debug, Clone, Copy)]
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

impl<NonTerminalId: Hash, Extras> Hash for NonTerminal<NonTerminalId, Extras> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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
pub struct Body<SymbolType> {
    body: Vec<SymbolType>,
}

impl<SymbolType, Idx> IndexMut<Idx> for Body<SymbolType>
where
    Idx: SliceIndex<[SymbolType]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.body.index_mut(index)
    }
}

impl<SymbolType, Idx> Index<Idx> for Body<SymbolType>
where
    Idx: SliceIndex<[SymbolType]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.body.index(index)
    }
}

impl<SymbolType> DerefMut for Body<SymbolType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.body.deref_mut()
    }
}

impl<SymbolType> Deref for Body<SymbolType> {
    type Target = [SymbolType];

    fn deref(&self) -> &Self::Target {
        self.body.deref()
    }
}

impl<SymbolType> Display for Body<SymbolType>
where
    SymbolType: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.body.iter().format(", "))
    }
}

impl<SymbolType> Body<SymbolType> {
    pub fn new(body: Vec<SymbolType>) -> Self {
        Self { body }
    }

    pub fn iter(&self) -> std::slice::Iter<SymbolType> {
        self.body.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<SymbolType> {
        self.body.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<SymbolType> {
        self.body.into_iter()
    }
}

pub struct Production<ProductionId, HeadType, BodySymbol, Extras = ()> {
    id: ProductionId,
    head: HeadType,
    body: Body<BodySymbol>,
    extras: Extras,
}

impl<ProductionId: PartialEq, HeadType, BodySymbol, Extras> PartialEq
    for Production<ProductionId, HeadType, BodySymbol, Extras>
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<ProductionId: Hash, HeadType, BodySymbol, Extras> Hash for Production<ProductionId, HeadType, BodySymbol, Extras> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<ProductionId, HeadType, BodySymbol, Extras>
    Production<ProductionId, HeadType, BodySymbol, Extras>
{
    pub fn new(id: ProductionId, head: HeadType, body: Body<BodySymbol>, extras: Extras) -> Self {
        Self {
            id,
            head,
            body,
            extras,
        }
    }

    pub fn id(&self) -> &ProductionId {
        &self.id
    }

    pub fn head(&self) -> &HeadType {
        &self.head
    }

    pub fn body(&self) -> &Body<BodySymbol> {
        &self.body
    }

    pub fn arity(&self) -> usize {
        self.body().len()
    }

    pub fn extras(&self) -> &Extras {
        &self.extras
    }
}

impl EnrichedBaseProduction {
    pub fn into_production(
        self,
        tokens: &[EnrichedToken],
        non_terminals: &[EnrichedNonTerminal],
    ) -> EnrichedProduction {
        EnrichedProduction::new(
            self.id,
            self.head,
            self.body
                .into_iter()
                .map(|ident| {
                    tokens
                        .iter()
                        .position(|tok| tok.ident() == &ident)
                        .map(|id| EnrichedSymbol::Token(tokens[id].clone()))
                        .or_else(|| {
                            non_terminals
                                .iter()
                                .position(|nt| nt.ident() == &ident)
                                .map(|id| EnrichedSymbol::NonTerminal(non_terminals[id].clone()))
                        })
                        .expect("ident is neither a non terminal nor a token")
                })
                .collect(),
            self.id()
        )
    }
}

pub struct Grammar<TokenType, NonTerminalType, ProductionType, Extras = ()> {
    tokens: Vec<TokenType>,
    non_terminals: Vec<NonTerminalType>,
    start_symbol: usize,
    productions: Vec<ProductionType>,
    extras: Extras,
}

impl<TokenType, NonTerminalType, ProductionType, Extras>
    Grammar<TokenType, NonTerminalType, ProductionType, Extras>
{
    pub fn new(
        tokens: Vec<TokenType>,
        non_terminals: Vec<NonTerminalType>,
        start_symbol: usize,
        productions: Vec<ProductionType>,
        extras: Extras,
    ) -> Self {
        Self {
            tokens,
            non_terminals,
            start_symbol,
            productions,
            extras,
        }
    }

    pub fn tokens(&self) -> &Vec<TokenType> {
        &self.tokens
    }

    pub fn token_count(&self) -> usize {
        self.tokens().len()
    }

    pub fn non_terminals(&self) -> &Vec<NonTerminalType> {
        &self.non_terminals
    }

    pub fn non_terminal_count(&self) -> usize {
        self.non_terminals().len()
    }

    pub fn start_symbol(&self) -> &NonTerminalType {
        self.non_terminals().get(self.start_symbol).unwrap()
    }

    pub fn productions(&self) -> &Vec<ProductionType> {
        &self.productions
    }

    pub fn extras(&self) -> &Extras {
        &self.extras
    }
}

impl From<EnrichedGrammar> for SymbolicGrammar {
    fn from(value: EnrichedGrammar) -> Self {
        let tokens = value
            .tokens
            .into_iter()
            .enumerate()
            .map(|(id, enr)| SymbolicToken::new(id, enr))
            .collect_vec();
        let non_terminals = value
            .non_terminals
            .into_iter()
            .enumerate()
            .map(|(id, enr)| SymbolicNonTerminal::new(id, enr))
            .collect_vec();
        let productions = value
            .productions
            .into_iter()
            .enumerate()
            .map(|(id, enr)| {
                let head_id = *non_terminals
                    .iter()
                    .find(|nt| nt.extras().id() == enr.head().id())
                    .unwrap()
                    .id();
                let body = enr
                    .body
                    .into_iter()
                    .map(|sym| match sym {
                        EnrichedSymbol::Token(tok) => SymbolicSymbol::Token(SymbolicToken::new(
                            *tokens
                                .iter()
                                .find(|tok2| tok2.extras().id() == tok.id())
                                .unwrap()
                                .id(),
                            tok,
                        )),
                        Symbol::NonTerminal(nt) => {
                            SymbolicSymbol::NonTerminal(SymbolicNonTerminal::new(
                                *non_terminals
                                    .iter()
                                    .find(|nt2| nt2.extras().id() == nt.id())
                                    .unwrap()
                                    .id(),
                                nt,
                            ))
                        }
                    })
                    .collect_vec();

                SymbolicProduction::new(
                    id,
                    SymbolicNonTerminal::new(head_id, enr.head),
                    Body::new(body),
                    enr.id,
                )
            })
            .collect_vec();
        SymbolicGrammar::new(tokens, non_terminals, value.start_symbol, productions, ())
    }
}
