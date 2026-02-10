use semasia::*;

use crate::abcs::Parser;

#[grammar]
mod abcs {
    use semasia::*;

    #[non_terminal]
    #[start_symbol]
    #[derive(Debug)]
    pub struct S(Vec<A>, Option<B>, usize);

    #[token("a")]
    #[derive(Debug)]
    pub struct A;

    #[token("b")]
    #[derive(Debug)]
    pub struct B;

    #[token("c")]
    #[derive(Debug)]
    pub struct C;

    #[token("d")]
    #[derive(Debug)]
    pub struct D;

    ebnf!(P0, S -> (A*, CorD { C, D }), |(a, c_or_d)| {
        S(a, None, match c_or_d { CorD::C(_) => 0, CorD::D(_) => 1})
    });
}

fn main() {
    let res = Parser::lex_parse("aaaaad");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
