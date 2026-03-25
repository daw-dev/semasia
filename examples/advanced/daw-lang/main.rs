mod grammar;

use grammar::language;

fn main() {
    let file = include_str!("main.c");
    println!("compiling the following script:");
    print!("{file}");
    let result = language::Parser::lex_parse_default_ctx(file);
    match result {
        Ok((program, _)) => {
            println!("{program:?}");
        }
        Err(err) => eprintln!("{err}"),
    }
}
