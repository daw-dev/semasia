use semasia::*;

#[grammar]
mod scream {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub type Scream = Vec<A>;

    #[token("a")]
    #[derive(Debug)]
    pub struct A;

    ebnf!(LongScream, Scream -> A*);
}

use scream::Parser;

fn main() {
    let scream = Parser::lex_parse("aaaaaaaaaaaaaaaaa").expect("couldn't parse");
    println!("{scream:?}");
}
