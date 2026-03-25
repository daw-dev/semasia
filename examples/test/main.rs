use semasia::grammar;

#[grammar]
mod abcs {
    use semasia::*;

    #[non_terminal]
    #[start_symbol]
    #[derive(Debug)]
    pub struct A;
    
    #[non_terminal]
    pub struct B;

    production!(P0, A -> B, |_| A);
    production!(P1, B -> A, |_| B);
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("aaaaad");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
