use crate::results::{
    LexError, LexParseError, ParseEof, ParseEofError, ParseEofErrorReason, ParseError,
    ParseOneError, ParseToken, ParseTokenError, ParseTokenErrorReason,
};
use logos::Logos;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Range,
};

mod actions;
pub mod dummy;
pub mod results;
mod traits;

pub use actions::*;
pub use traits::*;

#[derive(Debug)]
pub enum Symbol<NonTerminal, Token> {
    NonTerminal(NonTerminal),
    Token(Token),
}

impl<NonTerminal: Display, Token: Display> Display for Symbol<NonTerminal, Token> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::NonTerminal(nt) => write!(f, "NonTerminal({nt})"),
            Symbol::Token(tok) => write!(f, "Token({tok})"),
        }
    }
}

#[derive(Debug)]
pub struct Stacks<NonTerminal, Token> {
    pub state_stack: Vec<usize>,
    pub symbol_stack: Vec<Symbol<NonTerminal, Token>>,
}

impl<NonTerminal, Token> Stacks<NonTerminal, Token> {
    pub fn new() -> Self {
        Self {
            state_stack: vec![0],
            symbol_stack: Vec::new(),
        }
    }

    pub fn current_state(&self) -> usize {
        *self.state_stack.last().expect("state stack is empty!")
    }

    pub fn shift(&mut self, new_state: usize, token: Token) {
        self.state_stack.push(new_state);
        self.symbol_stack.push(Symbol::Token(token));
    }

    pub fn goto(&mut self, new_state: usize, non_terminal: NonTerminal) {
        self.state_stack.push(new_state);
        self.symbol_stack.push(Symbol::NonTerminal(non_terminal));
    }
}

impl<NonTerminal, Terminal> Default for Stacks<NonTerminal, Terminal> {
    fn default() -> Self {
        Self {
            state_stack: Default::default(),
            symbol_stack: Default::default(),
        }
    }
}

pub struct Parser<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx> {
    stacks: Stacks<NonTerminal, Token>,
    ctx: Ctx,
    phantom_data: PhantomData<(StartSymbol, Prod, Tab)>,
}

impl<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx> Debug
    for Parser<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>
where
    Stacks<NonTerminal, Token>: Debug,
    Ctx: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser")
            .field("stacks", &self.stacks)
            .field("ctx", &self.ctx)
            .finish()
    }
}

pub type ParseSource = ();

pub type ParseResult<Parser, NonTerminal, Token, ReturnType> =
    Result<ReturnType, ParseError<Parser, NonTerminal, Token, usize, ParseSource>>;

pub type LexParseResult<'source, Parser, NonTerminal, Token, ReturnType> = Result<
    ReturnType,
    LexParseError<
        LexError<'source, Parser, Token>,
        ParseError<
            Parser,
            NonTerminal,
            Token,
            Range<usize>,
            &'source <Token as Logos<'source>>::Source,
        >,
    >,
>;

impl<
    NonTerminal,
    Token,
    StartSymbol: From<NonTerminal>,
    Prod: Reduce<NonTerminal, Token, Ctx>,
    Tab: Tables<NonTerminal, Token, Prod>,
    Ctx,
> Parser<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>
{
    pub fn with_ctx(ctx: Ctx) -> Self {
        Self {
            stacks: Stacks::new(),
            ctx,
            phantom_data: PhantomData,
        }
    }

    pub fn default_ctx() -> Self
    where
        Ctx: Default,
    {
        Self::with_ctx(Default::default())
    }

    pub fn current_state(&self) -> usize {
        self.stacks.current_state()
    }

    fn parse_token(
        &mut self,
        token: Token,
    ) -> Result<ParseToken<Token>, ParseTokenErrorReason<NonTerminal, Token>> {
        let current_state = self.current_state();
        match Tab::query_token_table(current_state, &token) {
            Some(TokenAction::Shift(new_state)) => {
                self.stacks.shift(new_state, token);
                Ok(ParseToken::Shifted)
            }
            Some(TokenAction::Reduce(prod)) => {
                let head = prod.reduce(&mut self.ctx, &mut self.stacks);
                let new_current_state = self.current_state();
                let Some(next_state) = Tab::query_goto_table(new_current_state, &head) else {
                    return Err(ParseTokenErrorReason::GotoNotFound {
                        leftover_non_terminal: head,
                    });
                };
                self.stacks.goto(next_state, head);
                Ok(ParseToken::Reduced {
                    leftover_token: token,
                })
            }
            None => Err(ParseTokenErrorReason::ActionNotFound {
                leftover_token: token,
            }),
        }
    }

    pub fn consume_token(
        &mut self,
        mut token: Token,
    ) -> Result<(), ParseTokenErrorReason<NonTerminal, Token>> {
        loop {
            match self.parse_token(token) {
                Ok(ParseToken::Shifted) => {
                    return Ok(());
                }
                Ok(ParseToken::Reduced { leftover_token }) => {
                    token = leftover_token;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    fn parse_eof(&mut self) -> Result<ParseEof, ParseEofErrorReason<NonTerminal>> {
        let current_state = self.current_state();
        match Tab::query_eof_table(current_state) {
            Some(EofAction::Reduce(prod)) => {
                let head = prod.reduce(&mut self.ctx, &mut self.stacks);
                let new_current_state = self.current_state();
                let Some(next_state) = Tab::query_goto_table(new_current_state, &head) else {
                    return Err(ParseEofErrorReason::GotoNotFound {
                        leftover_non_terminal: head,
                    });
                };
                self.stacks.goto(next_state, head);
                Ok(ParseEof::Reduced)
            }
            Some(EofAction::Accept) => Ok(ParseEof::Accepted),
            None => Err(ParseEofErrorReason::ActionNotFound),
        }
    }

    pub fn consume_eof(&mut self) -> Result<StartSymbol, ParseEofError<NonTerminal>>
    where
        StartSymbol: From<NonTerminal>,
    {
        loop {
            match self.parse_eof() {
                Ok(ParseEof::Accepted) => {
                    break;
                }
                Ok(ParseEof::Reduced) => {
                    continue;
                }
                Err(err) => {
                    return Err(ParseEofError::new(err));
                }
            }
        }

        let Symbol::NonTerminal(non_terminal) = self.stacks.symbol_stack.pop().unwrap() else {
            unreachable!()
        };

        Ok(non_terminal.into())
    }

    pub fn do_parse(
        mut self,
        tokens: impl IntoIterator<Item = Token>,
    ) -> ParseResult<Self, NonTerminal, Token, (StartSymbol, Ctx)> {
        for (span, token) in tokens.into_iter().enumerate() {
            if let Err(err) = self
                .consume_token(token)
                .map_err(|err| ParseTokenError::new(err, span))
                .map_err(ParseOneError::ParseTokenError)
            {
                return Err(ParseError::new(self, err, ()));
            }
        }

        match self.consume_eof().map_err(ParseOneError::ParseEofError) {
            Err(err) => Err(ParseError::new(self, err, ())),
            Ok(res) => Ok((res, self.ctx)),
        }
    }

    pub fn parse_with_ctx(
        ctx: Ctx,
        tokens: impl IntoIterator<Item = Token>,
    ) -> ParseResult<Self, NonTerminal, Token, (StartSymbol, Ctx)> {
        Self::with_ctx(ctx).do_parse(tokens)
    }

    pub fn parse_default_ctx(
        tokens: impl IntoIterator<Item = Token>,
    ) -> ParseResult<Self, NonTerminal, Token, (StartSymbol, Ctx)>
    where
        Ctx: Default,
    {
        Self::default_ctx().do_parse(tokens)
    }

    pub fn do_lex_parse<'source>(
        mut self,
        source: &'source Token::Source,
    ) -> LexParseResult<'source, Self, NonTerminal, Token, (StartSymbol, Ctx)>
    where
        Token: Logos<'source>,
        Token::Extras: Default,
    {
        for (token, span) in Token::lexer(source).spanned() {
            let mut token = match token {
                Ok(token) => token,
                Err(err) => {
                    return Err(LexParseError::LexError(LexError::new(
                        self, err, span, source,
                    )));
                }
            };

            loop {
                match self.parse_token(token) {
                    Ok(ParseToken::Shifted) => {
                        break;
                    }
                    Ok(ParseToken::Reduced { leftover_token }) => {
                        token = leftover_token;
                    }
                    Err(err) => {
                        return Err(LexParseError::ParseError(ParseError::new(
                            self,
                            ParseOneError::ParseTokenError(ParseTokenError::new(err, span)),
                            source,
                        )));
                    }
                }
            }
        }

        loop {
            match self.parse_eof() {
                Ok(ParseEof::Accepted) => {
                    break;
                }
                Ok(ParseEof::Reduced) => {
                    continue;
                }
                Err(err) => {
                    return Err(LexParseError::ParseError(ParseError::new(
                        self,
                        ParseOneError::ParseEofError(ParseEofError::new(err)),
                        source,
                    )));
                }
            }
        }

        let Symbol::NonTerminal(non_terminal) = self.stacks.symbol_stack.pop().unwrap() else {
            unreachable!()
        };

        Ok((non_terminal.into(), self.ctx))
    }

    pub fn lex_parse_with_ctx<'source>(
        ctx: Ctx,
        source: &'source Token::Source,
    ) -> LexParseResult<'source, Self, NonTerminal, Token, (StartSymbol, Ctx)>
    where
        Token: Logos<'source>,
        Token::Extras: Default,
    {
        Self::with_ctx(ctx).do_lex_parse(source)
    }

    pub fn lex_parse_default_ctx<'source>(
        source: &'source Token::Source,
    ) -> LexParseResult<'source, Self, NonTerminal, Token, (StartSymbol, Ctx)>
    where
        Token: Logos<'source>,
        Token::Extras: Default,
        Ctx: Default,
    {
        Self::default_ctx().do_lex_parse(source)
    }
}

impl<
    NonTerminal,
    Token,
    StartSymbol: From<NonTerminal>,
    Prod: Reduce<NonTerminal, Token, ()>,
    Tab: Tables<NonTerminal, Token, Prod>,
> Parser<NonTerminal, Token, StartSymbol, Prod, Tab, ()>
{
    pub fn new() -> Self {
        Self::with_ctx(())
    }

    pub fn parse(
        tokens: impl Iterator<Item = Token>,
    ) -> ParseResult<Self, NonTerminal, Token, StartSymbol> {
        Self::new().do_parse(tokens).map(|ok| ok.0)
    }

    pub fn lex_parse<'source>(
        source: &'source Token::Source,
    ) -> LexParseResult<'source, Self, NonTerminal, Token, StartSymbol>
    where
        Token: Logos<'source>,
        Token::Extras: Default,
    {
        Self::new().do_lex_parse(source).map(|ok| ok.0)
    }
}
