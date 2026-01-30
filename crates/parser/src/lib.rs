use crate::results::{
    LexError, LexParseError, ParseEofError, ParseError, ParseOne, ParseOneEof, ParseOneError,
    ParseTokenError,
};
use logos::Logos;
use std::marker::PhantomData;

mod actions;
mod results;
mod traits;

pub use actions::*;
pub use traits::*;

#[derive(Debug)]
pub enum Symbol<NonTerminal, Token> {
    NonTerminal(NonTerminal),
    Token(Token),
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

#[derive(Debug)]
pub struct Parser<
    NonTerminal: Into<StartSymbol>,
    Token,
    StartSymbol,
    Prod: Reduce<NonTerminal, Token, Ctx>,
    Tab: Tables<NonTerminal, Token, Prod>,
    Ctx,
> {
    stacks: Stacks<NonTerminal, Token>,
    ctx: Ctx,
    phantom_data: PhantomData<(StartSymbol, Prod, Tab)>,
}

impl<
    NonTerminal: Into<StartSymbol>,
    Token,
    StartSymbol,
    Prod: Reduce<NonTerminal, Token, Ctx>,
    Tab: Tables<NonTerminal, Token, Prod>,
    Ctx,
> Parser<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>
{
    fn new(ctx: Ctx) -> Self {
        Self {
            stacks: Stacks::new(),
            ctx,
            phantom_data: PhantomData,
        }
    }

    fn parse_token(
        &mut self,
        token: Token,
    ) -> Result<ParseOne<Token>, ParseTokenError<NonTerminal, Token>> {
        let current_state = self.stacks.current_state();
        match Tab::query_token_table(current_state, &token) {
            Some(TokenAction::Shift(new_state)) => {
                self.stacks.shift(new_state, token);
                Ok(ParseOne::Shifted)
            }
            Some(TokenAction::Reduce(prod)) => {
                let head = prod.reduce(&mut self.ctx, &mut self.stacks);
                let new_current_state = self.stacks.current_state();
                let Some(next_state) = Tab::query_goto_table(new_current_state, &head) else {
                    return Err(ParseTokenError::GotoNotFound {
                        leftover_non_terminal: head,
                    });
                };
                self.stacks.goto(next_state, head);
                Ok(ParseOne::Reduced {
                    leftover_token: token,
                })
            }
            None => Err(ParseTokenError::ActionNotFound {
                leftover_token: token,
            }),
        }
    }

    fn parse_eof(&mut self) -> Result<ParseOneEof, ParseEofError<NonTerminal>> {
        let current_state = self.stacks.current_state();
        match Tab::query_eof_table(current_state) {
            Some(EofAction::Reduce(prod)) => {
                let head = prod.reduce(&mut self.ctx, &mut self.stacks);
                let new_current_state = self.stacks.current_state();
                let Some(next_state) = Tab::query_goto_table(new_current_state, &head) else {
                    return Err(ParseEofError::GotoNotFound {
                        leftover_non_terminal: head,
                    });
                };
                self.stacks.goto(next_state, head);
                Ok(ParseOneEof::Reduced)
            }
            Some(EofAction::Accept) => Ok(ParseOneEof::Accepted),
            None => Err(ParseEofError::ActionNotFound),
        }
    }

    pub fn parse_with_ctx(
        ctx: Ctx,
        tokens: impl IntoIterator<Item = Token>,
    ) -> Result<StartSymbol, ParseError<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>> {
        let mut parser = Self::new(ctx);
        for mut token in tokens.into_iter() {
            loop {
                match parser.parse_token(token) {
                    Ok(ParseOne::Shifted) => {
                        break;
                    }
                    Ok(ParseOne::Reduced { leftover_token }) => {
                        token = leftover_token;
                    }
                    Err(err) => {
                        return Err(ParseError::new(parser, ParseOneError::ParseTokenError(err)));
                    }
                }
            }
        }

        loop {
            match parser.parse_eof() {
                Ok(ParseOneEof::Accepted) => {
                    break;
                }
                Ok(ParseOneEof::Reduced) => {
                    continue;
                }
                Err(err) => return Err(ParseError::new(parser, ParseOneError::ParseEofError(err))),
            }
        }

        let Symbol::NonTerminal(non_terminal) = parser.stacks.symbol_stack.pop().unwrap() else {
            unreachable!()
        };

        Ok(non_terminal.into())
    }

    pub fn parse_default_ctx(
        tokens: impl IntoIterator<Item = Token>,
    ) -> Result<StartSymbol, ParseError<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>>
    where
        Ctx: Default,
    {
        Self::parse_with_ctx(Default::default(), tokens)
    }

    pub fn lex_parse_with_ctx<'source>(
        ctx: Ctx,
        source: &'source <Token as Logos<'source>>::Source,
    ) -> Result<StartSymbol, LexParseError<'source, NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>>
    where
        Token: Logos<'source>,
        Token::Extras: Default,
    {
        let mut parser = Self::new(ctx);
        for token in Token::lexer(source) {
            let mut token = match token {
                Ok(token) => token,
                Err(err) => return Err(LexParseError::LexError(LexError::new(parser, err))),
            };

            loop {
                match parser.parse_token(token) {
                    Ok(ParseOne::Shifted) => {
                        break;
                    }
                    Ok(ParseOne::Reduced { leftover_token }) => {
                        token = leftover_token;
                    }
                    Err(err) => {
                        return Err(LexParseError::ParseError(ParseError::new(
                            parser,
                            ParseOneError::ParseTokenError(err),
                        )));
                    }
                }
            }
        }

        loop {
            match parser.parse_eof() {
                Ok(ParseOneEof::Accepted) => {
                    break;
                }
                Ok(ParseOneEof::Reduced) => {
                    continue;
                }
                Err(err) => {
                    return Err(LexParseError::ParseError(ParseError::new(
                        parser,
                        ParseOneError::ParseEofError(err),
                    )));
                }
            }
        }

        let Symbol::NonTerminal(non_terminal) = parser.stacks.symbol_stack.pop().unwrap() else {
            unreachable!()
        };

        Ok(non_terminal.into())
    }

    pub fn lex_parse_default_ctx<'source>(
        source: &'source <Token as Logos<'source>>::Source,
    ) -> Result<StartSymbol, LexParseError<'source, NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>>
    where
        Token: Logos<'source>,
        <Token as Logos<'source>>::Extras: Default,
        Ctx: Default,
    {
        Self::lex_parse_with_ctx(Default::default(), source)
    }
}

impl<
    NonTerminal: Into<StartSymbol>,
    Token,
    StartSymbol,
    Prod: Reduce<NonTerminal, Token, ()>,
    Tab: Tables<NonTerminal, Token, Prod>,
> Parser<NonTerminal, Token, StartSymbol, Prod, Tab, ()>
{
    pub fn parse(
        tokens: impl Iterator<Item = Token>,
    ) -> Result<StartSymbol, ParseError<NonTerminal, Token, StartSymbol, Prod, Tab, ()>> {
        Self::parse_with_ctx((), tokens)
    }

    pub fn lex_parse<'source>(
        source: &'source <Token as Logos<'source>>::Source,
    ) -> Result<StartSymbol, LexParseError<'source, NonTerminal, Token, StartSymbol, Prod, Tab, ()>>
    where
        Token: Logos<'source>,
        <Token as Logos<'source>>::Extras: Default,
    {
        Self::lex_parse_with_ctx((), source)
    }
}
