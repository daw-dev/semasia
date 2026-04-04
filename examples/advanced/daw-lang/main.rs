mod grammar;

use grammar::language;
use parser::{results::LexParseError};

fn main() {
    let file = include_str!("main.c");
    println!("compiling the following script:");
    print!("{file}");
    let result = language::Parser::lex_parse_default_ctx(file);
    match result {
        Ok((program, ctx)) => {
            println!("{program}");
            println!("ctx is {ctx:?}");
        }
        Err(err) => {
            eprintln!("{err}");
            if let LexParseError::ParseError(error) = err {
                eprintln!("{:?}", error.parser);
            }
        },
    }
}

#[test]
fn test_if_statement() -> Result<(), Box<dyn std::error::Error>>{
    fn lex(source: &str) -> Token {
        Token::lexer(source).next().unwrap().unwrap()
    }

    use parser::Tables;
    use language::Token;
    use logos::Logos;

    let mut parser = language::Parser::default_ctx();
    println!("{:?}", language::Tables::tokens_in_state(parser.current_state()));
    parser.consume_token(lex("if"))?;
    // println!("{parser:?}");
    parser.consume_token(lex("("))?;
    // println!("{parser:?}");
    parser.consume_token(lex("123"))?;
    // println!("{parser:?}");
    parser.consume_token(lex(")"))?;
    // println!("{parser:?}");
    // parser.consume_token(lex("print"))?;
    // parser.consume_token(lex("("))?;
    parser.consume_token(lex("123"))?;
    // parser.consume_token(lex(","))?;
    // parser.consume_token(lex("321"))?;
    // parser.consume_token(lex(")"))?;
    parser.consume_token(lex(";"))?;
    println!("{parser:?}");
    println!("about to consume else");
    println!("{:?}", language::Tables::tokens_in_state(parser.current_state()));
    parser.consume_token(lex("else"))?;
    println!("ELSE CONSUMED");
    println!("{:?}", language::Tables::tokens_in_state(parser.current_state()));
    parser.consume_token(lex(")"))?;
    println!("{:?}", language::Tables::tokens_in_state(parser.current_state()));
    parser.consume_token(lex("321"))?;
    println!("{parser:?}");
    println!("{:?}", language::Tables::tokens_in_state(parser.current_state()));
    parser.consume_token(lex("print"))?;
    parser.consume_token(lex("("))?;
    parser.consume_token(lex("123"))?;
    parser.consume_token(lex(")"))?;
    parser.consume_token(lex(";"))?;

    parser.consume_token(lex("print"))?;
    parser.consume_token(lex("("))?;
    parser.consume_token(lex("123"))?;
    parser.consume_token(lex(")"))?;
    parser.consume_token(lex(";"))?;
    parser.consume_eof()?;
    // QUESTO VUOL DIRE CHE NON FUNZIONA L'EBNF

    Ok(())
}
