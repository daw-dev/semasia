use semasia::*;

#[grammar]
mod laugh {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    #[derive(Debug)]
    pub struct Laugh(Vec<A>);

    #[derive(Debug)]
    #[token("a")]
    #[token("A")]
    pub struct A;

    #[token("h")]
    #[token("H")]
    pub struct H;

    ebnf!(LaughProd: Laugh -> #[separator(H)] Vec<A>, |aa| Laugh(aa));
}

fn main() {
    let result = laugh::Parser::lex_parse("ahahaha").expect("couldn't parse");
    println!("{result:?}");
}
