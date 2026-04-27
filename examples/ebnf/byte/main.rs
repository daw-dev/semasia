#[semasia::grammar]
pub mod byte {
    use semasia::*;

    #[non_terminal]
    #[start_symbol]
    pub type Byte = [Bit; 8];

    #[token("1", |_| true)]
    #[token("0", |_| false)]
    pub type Bit = bool;

    #[token("_")]
    pub struct Underscore;

    ebnf!(ConstructByte: Byte -> #[separator(Option<Underscore>)] [Bit; 8]);
}

fn main() {
    let result = byte::Parser::lex_parse("100_101_00").expect("couldn't parse");
    println!("{result:?}");
}
