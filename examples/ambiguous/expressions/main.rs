use semasia::*;

#[grammar]
mod ambiguous {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub type Expression = usize;

    #[token(regex = r"\d+")]
    pub type Number = usize;

    #[token("+")]
    #[left_associative]
    #[precedence(0)]
    pub struct Plus;

    #[token("-")]
    #[left_associative]
    #[precedence(0)]
    pub struct Minus;

    #[token("*")]
    #[left_associative]
    #[precedence(1)]
    pub struct Times;

    #[token("/")]
    #[left_associative]
    #[precedence(1)]
    pub struct Division;

    #[token("^")]
    #[right_associative]
    #[precedence(1)]
    pub struct Power;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosePar;

    production!(P1, Expression -> (Expression, Plus, Expression), |(e1, _, e2)| e1 + e2);

    production!(P2, Expression -> (Expression, Minus, Expression), |(e1, _, e2)| e1 - e2);

    production!(P3, Expression -> (Expression, Times, Expression), |(e1, _, e2)| e1 * e2);

    production!(P4, Expression -> (Expression, Division, Expression), |(e1, _, e2)| e1 * e2);

    production!(P5, Expression -> (Expression, Power, Expression), |(e1, _, e2)| e1.pow(e2 as u32));

    production!(P6, Expression -> (OpenPar, Expression, ClosePar), |(_, e, _)| e);

    production!(P7, Expression -> Number);
}

use ambiguous::*;

fn main() {
    let res = Parser::lex_parse("15 - 2 * 2 ^ 2").expect("couldn't parse");

    println!("{res}");
}
