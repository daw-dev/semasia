use semasia::*;

#[grammar]
mod abcs {
    use semasia::*;

    #[non_terminal]
    #[start_symbol]
    #[derive(Debug)]
    pub struct S(Vec<A>, Option<B>, C);

    #[token("a")]
    #[derive(Debug)]
    pub struct A;

    #[token("b")]
    #[derive(Debug)]
    pub struct B;

    #[token("c")]
    #[derive(Debug)]
    pub struct C;

    ebnf!(P0: S -> (Vec<A>, Option<B>, C), |(a, b, c)| {
        S(a, b, c)
    });
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("aaaaabc");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
