mod grammar;

use grammar::language;

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
            eprintln!("{err:?}");
        },
    }
}

#[test]
fn if_statement_test() {
    use language::*;

    let result = language::Parser::parse_default_ctx([
        Token::Ident("int".to_string()),
        Token::Ident("main".to_string()),
        Token::OpenPar(OpenPar),
        Token::ClosePar(ClosePar),
        Token::If(If),
        Token::OpenPar(OpenPar),
        Token::Ident("a".to_string()),
    ]);
}
