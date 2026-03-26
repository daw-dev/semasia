use semasia::*;

#[grammar]
#[logos(skip r"\s+")]
mod ambiguous {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub type Expression = f64;

    #[regex(r"\d+(\.\d+)?", parse)]
    pub type Number = f64;

    #[token("+")]
    #[left_associative]
    #[priority(0)]
    pub struct Plus;

    #[token("-")]
    #[left_associative]
    #[priority(0)]
    pub struct Minus;

    #[token("*")]
    #[left_associative]
    #[priority(1)]
    pub struct Times;

    #[token("/")]
    #[left_associative]
    #[priority(1)]
    pub struct DivisionOp;

    #[token("^")]
    #[right_associative]
    #[priority(2)]
    pub struct Power;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosePar;

    production!(Sum, Expression -> (Expression, Plus, Expression), |(e1, _, e2)| e1 + e2);
    production!(Difference, Expression -> (Expression, Minus, Expression), |(e1, _, e2)| e1 - e2);
    production!(Product, Expression -> (Expression, Times, Expression), |(e1, _, e2)| e1 * e2);
    production!(Division, Expression -> (Expression, DivisionOp, Expression), |(e1, _, e2)| e1 * e2);
    production!(Exponent, Expression -> (Expression, Power, Expression), |(e1, _, e2)| e1.powf(e2));
    production!(Parethesis, Expression -> (OpenPar, Expression, ClosePar), |(_, e, _)| e);
    #[priority(3)]
    production!(Negation, Expression -> (Minus, Expression), |(_, e)| -e);
    production!(ActualNumber, Expression -> Number);
}

use ambiguous::*;

fn main() {
    let res = Parser::lex_parse("-15 - 2 * -2 ^ 3");

    match res {
        Ok(res) => println!("result: {res:?}"),
        Err(err) => eprintln!("{err}"),
    }
}
