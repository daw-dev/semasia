use std::{fmt::Display, ops::Range};

use itertools::Itertools;
use logos::Logos;

use crate::{Parser, Reduce, Tables};

#[derive(Debug)]
pub enum ParseToken<Token> {
    Shifted,
    Reduced { leftover_token: Token },
}

#[derive(Debug)]
pub enum ParseEof {
    Reduced,
    Accepted,
}

#[derive(Debug)]
pub enum ParseTokenErrorReason<NonTerminal, Token> {
    ActionNotFound { leftover_token: Token },
    GotoNotFound { leftover_non_terminal: NonTerminal },
}

#[derive(Debug)]
pub struct ParseTokenError<NonTerminal, Token, Span> {
    reason: ParseTokenErrorReason<NonTerminal, Token>,
    span: Span,
}

impl<NonTerminal, Token, Span> ParseTokenError<NonTerminal, Token, Span> {
    pub fn new(reason: ParseTokenErrorReason<NonTerminal, Token>, span: Span) -> Self {
        Self { reason, span }
    }
}

#[derive(Debug)]
pub enum ParseEofErrorReason<NonTerminal> {
    ActionNotFound,
    GotoNotFound { leftover_non_terminal: NonTerminal },
}

#[derive(Debug)]
pub struct ParseEofError<NonTerminal> {
    reason: ParseEofErrorReason<NonTerminal>,
}

impl<NonTerminal> ParseEofError<NonTerminal> {
    pub fn new(reason: ParseEofErrorReason<NonTerminal>) -> Self {
        Self { reason }
    }
}

#[derive(Debug)]
pub enum ParseOneError<NonTerminal, Token, Span> {
    ParseTokenError(ParseTokenError<NonTerminal, Token, Span>),
    ParseEofError(ParseEofError<NonTerminal>),
}

#[derive(Debug)]
pub struct ParseError<Parser, NonTerminal, Token, Span, Source> {
    pub parser: Parser,
    pub parse_one_error: ParseOneError<NonTerminal, Token, Span>,
    pub source: Source,
}

impl<Parser, NonTerminal, Token, Span, Source>
    ParseError<Parser, NonTerminal, Token, Span, Source>
{
    pub fn new(
        parser: Parser,
        parse_one_error: ParseOneError<NonTerminal, Token, Span>,
        source: Source,
    ) -> Self {
        Self {
            parser,
            parse_one_error,
            source,
        }
    }
}

impl<'source, Parser, NonTerminal, Token> Display
    for ParseError<Parser, NonTerminal, Token, Range<usize>, &'source Token::Source>
where
    Token: Logos<'source> + Display,
    Token::Source: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "ParseError: after [{}] expected any of [{}]",
        //     self.parser.stacks.symbol_stack.iter().format(", "),
        //     Tab::tokens_in_state(self.parser.stacks.current_state())
        //         .iter()
        //         .format(", ")
        // )
        match &self.parse_one_error {
            ParseOneError::ParseTokenError(parse_token_error) => match &parse_token_error.reason {
                ParseTokenErrorReason::ActionNotFound { leftover_token } => {
                    writeln!(
                        f,
                        "unexpected token {leftover_token}, expected tokens are: ..."
                    )?;
                    writeln!(f, "{}", self.source)?;
                    write!(
                        f,
                        "{}{}",
                        " ".repeat(parse_token_error.span.start),
                        "^".repeat(parse_token_error.span.end - parse_token_error.span.start)
                    )
                }
                ParseTokenErrorReason::GotoNotFound {
                    leftover_non_terminal: _,
                } => unreachable!("correctly reduced a production, but no goto action found"),
            },
            ParseOneError::ParseEofError(parse_eof_error) => {
                write!(f, "parse_eof_error")
            }
        }
    }
}

#[derive(Debug)]
pub struct LexError<'source, Parser, Token: Logos<'source>> {
    pub parser: Parser,
    pub lexer_error: Token::Error,
    pub span: Range<usize>,
    pub source: &'source Token::Source,
}

impl<'source, Parser, Token: Logos<'source>> LexError<'source, Parser, Token> {
    pub fn new(
        parser: Parser,
        lexer_error: Token::Error,
        span: Range<usize>,
        source: &'source Token::Source,
    ) -> Self {
        Self {
            parser,
            lexer_error,
            span,
            source,
        }
    }
}

impl<'source, Parser, Token: Logos<'source>> Display for LexError<'source, Parser, Token>
where
    Token::Source: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Character not recognized:")?;
        if self.span.start > 15 {
            writeln!(f, "")
        } else {
            writeln!(f, "{}", self.source)?;
            write!(
                f,
                "{}{}",
                " ".repeat(self.span.start),
                "^".repeat(self.span.end - self.span.start)
            )
        }
    }
}

#[derive(Debug)]
pub enum LexParseError<LexError, ParseError> {
    LexError(LexError),
    ParseError(ParseError),
}

impl<LexError, ParseError> Display for LexParseError<LexError, ParseError>
where
    LexError: Display,
    ParseError: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexParseError::LexError(lex_error) => write!(f, "{lex_error}"),
            LexParseError::ParseError(parse_error) => write!(f, "{parse_error}"),
        }
    }
}
