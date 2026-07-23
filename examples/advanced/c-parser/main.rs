mod display_tree;
mod grammar;

use grammar::language;
use semasia_parser::results::LexParseError;
use ptree::print_tree;

fn main() {
    let file = include_str!("main.c");
    println!("compiling the following script:");
    print!("{file}");
    let result = language::Parser::lex_parse_default_ctx(file);
    match result {
        Ok((program, ctx)) => {
            print_tree(&program.build_tree()).expect("couldn't print tree");
            println!("ctx is {ctx:?}");
        }
        Err(err) => {
            eprintln!("{err}");
            if let LexParseError::ParseError(error) = err {
                eprintln!("{:?}", error.parser);
            }
        }
    }
}
