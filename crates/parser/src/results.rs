use std::{fmt::Display, ops::Range};

use colored::Colorize;
use itertools::Itertools;
use logos::Logos;

use crate::{Parser, Tables};

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

impl<'source, NonTerminal, Token, StartSymbol, Prod, Tab, Ctx> Display
    for ParseError<
        Parser<NonTerminal, Token, StartSymbol, Prod, Tab, Ctx>,
        NonTerminal,
        Token,
        Range<usize>,
        &'source Token::Source,
    >
where
    Token: Logos<'source, Source = str> + Display,
    Tab: Tables<NonTerminal, Token, Prod>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parse_one_error {
            ParseOneError::ParseTokenError(parse_token_error) => match &parse_token_error.reason {
                ParseTokenErrorReason::ActionNotFound { leftover_token } => {
                    writeln!(
                        f,
                        "{}{}{}",
                        "error".red().bold(),
                        ": unexpected token ".bold(),
                        leftover_token.to_string().bold(),
                    )?;
                    let (line, span, line_count) =
                        to_line_span(self.source, parse_token_error.span.clone());
                    let line_count_str = line_count.to_string();
                    let line_count_len = line_count_str.len();
                    writeln!(
                        f,
                        "{} {} {}",
                        line_count_str.blue().bold(),
                        "|".blue().bold(),
                        line
                    )?;
                    writeln!(
                        f,
                        "{}{}{}{}",
                        " ".repeat(line_count_len),
                        " | ".blue().bold(),
                        " ".repeat(span.start),
                        "^".repeat(span.end - span.start).red().bold()
                    )?;
                    write!(
                        f,
                        "{}{}{}expected tokens are {}",
                        " ".repeat(line_count_len),
                        " = ".blue().bold(),
                        "note: ".bold(),
                        Tab::tokens_in_state(self.parser.stacks.current_state())
                            .iter()
                            .format(", "),
                    )
                }
                ParseTokenErrorReason::GotoNotFound {
                    leftover_non_terminal: _,
                } => unreachable!("correctly reduced a production, but no goto action found"),
            },
            ParseOneError::ParseEofError(parse_eof_error) => match &parse_eof_error.reason {
                ParseEofErrorReason::ActionNotFound => {
                    writeln!(
                        f,
                        "{}{}",
                        "error".red().bold(),
                        ": unexpected end of source".bold(),
                    )?;
                    let (line, line_count) = last_line(self.source);
                    let line_count_str = line_count.to_string();
                    let line_count_len = line_count_str.len();
                    writeln!(
                        f,
                        "{} {} {}",
                        line_count_str.blue().bold(),
                        "|".blue().bold(),
                        line
                    )?;
                    writeln!(
                        f,
                        "{}{}{}{}",
                        " ".repeat(line_count_len),
                        " | ".blue().bold(),
                        " ".repeat(line.len()),
                        "^".red().bold()
                    )?;
                    write!(
                        f,
                        "{}{}{}expected tokens are {}",
                        " ".repeat(line_count_len),
                        " = ".blue().bold(),
                        "note: ".bold(),
                        Tab::tokens_in_state(self.parser.stacks.current_state())
                            .iter()
                            .format(", "),
                    )
                }
                ParseEofErrorReason::GotoNotFound {
                    leftover_non_terminal: _,
                } => unreachable!("correctly reduced a production, but no goto action found"),
            },
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

impl<'source, Parser, Token: Logos<'source, Source = str>> Display
    for LexError<'source, Parser, Token>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}",
            "error".red().bold(),
            ": unexpected character".bold()
        )?;
        let (line, span, line_count) = to_line_span(self.source, self.span.clone());
        let line_count_str = line_count.to_string();
        let line_count_len = line_count_str.len();
        writeln!(
            f,
            "{} {} {}",
            line_count_str.blue().bold(),
            "|".blue().bold(),
            line
        )?;
        write!(
            f,
            "{}{}{}{}",
            " ".repeat(line_count_len),
            " | ".blue().bold(),
            " ".repeat(span.start),
            "^".repeat(span.end - span.start).red().bold()
        )
    }
}

fn to_line_span(source: &str, mut span: Range<usize>) -> (&str, Range<usize>, usize) {
    for (line_count, line) in source.split('\n').enumerate() {
        let line_len = line.len();
        if span.start < line_len {
            return (line, span, line_count + 1);
        }
        span.start -= line_len + 1;
        span.end -= line_len + 1;
    }
    unreachable!()
}

fn last_line(source: &str) -> (&str, usize) {
    source
        .split('\n')
        .enumerate()
        .last()
        .map(|(line_count, line)| (line, line_count + 1))
        .unwrap()
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
