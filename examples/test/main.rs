use semasia::grammar;

#[grammar]
mod abcs {
    use semasia::production;

    #[start_symbol]
    #[non_terminal]
    pub type S = Box<B>;

    #[token("a")]
    #[derive(Debug)]
    pub struct A;

    #[token("b")]
    #[derive(Debug)]
    pub struct B;

    production!(P: S -> B);
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("b");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
