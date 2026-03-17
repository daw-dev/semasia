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
    #[associativity(Left)]
    #[precedence(0)]
    pub struct Plus;

    #[token("-")]
    #[associativity(Left)]
    #[precedence(0)]
    pub struct Minus;

    #[token("*")]
    #[associativity(Left)]
    #[precedence(1)]
    pub struct Times;

    #[token("/")]
    #[associativity(Left)]
    #[precedence(1)]
    pub struct DivisionOp;

    #[token("^")]
    #[associativity(Right)]
    #[precedence(2)]
    pub struct Power;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosePar;

    production!(Sum, Expression -> (Expression, Plus, Expression), |(e1, _, e2)| e1 + e2);

    production!(Difference, Expression -> (Expression, Minus, Expression), |(e1, _, e2)| e1 - e2);

    production!(Product, Expression -> (Expression, Times, Expression), |(e1, _, e2)| e1 * e2);

    production!(Division, Expression -> (Expression, DivisionOp, Expression), |(e1, _, e2)| e1 * e2);

    production!(Exponent, Expression -> (Expression, Power, Expression), |(e1, _, e2)| e1.pow(e2 as u32));

    production!(Parethesis, Expression -> (OpenPar, Expression, ClosePar), |(_, e, _)| e);

    production!(ActualNumber, Expression -> Number);
}

use ambiguous::*;

fn main() {
    let res = Parser::lex_parse("15 - 2 * 2 ^ 2").expect("couldn't parse");

    println!("{res}");
}
