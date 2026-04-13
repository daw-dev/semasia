use semasia::grammar;

#[grammar]
mod abcs {
    use auto_productions::auto_productions;
    use semasia::production;

    #[non_terminal]
    pub type S = Box<B>;

    #[token("a")]
    #[derive(Debug)]
    pub struct A;

    #[token("b")]
    #[derive(Debug)]
    pub struct B;

    production!(P: S -> B);

    #[auto_productions]
    #[non_terminal]
    #[start_symbol]
    #[derive(Debug)]
    pub enum Test {
        First(A, B, Box<B>),
        Second(S, A, B),
    }
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("bab");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
