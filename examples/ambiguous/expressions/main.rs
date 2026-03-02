use semasia::*;

#[grammar]
mod ambiguous {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub type E = usize;

    #[token(regex = r"\d+")]
    pub type Id = usize;

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

    production!(P1, E -> (E, Plus, E), |(e1, _, e2)| e1 + e2);

    production!(P2, E -> (E, Minus, E), |(e1, _, e2)| e1 - e2);

    production!(P3, E -> (E, Times, E), |(e1, _, e2)| e1 * e2);

    production!(P4, E -> (E, Division, E), |(e1, _, e2)| e1 * e2);

    production!(P5, E -> (E, Power, E), |(e1, _, e2)| e1.pow(e2 as u32));

    production!(P6, E -> (OpenPar, E, ClosePar), |(_, e, _)| e);

    production!(P7, E -> Id);
}

use ambiguous::*;

fn main() {
    let res = Parser::lex_parse("15 - 2 * 2 ^ 2").expect("couldn't parse");

    println!("{res}");
}
