use static_sdd::*;

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

fn main() {
    let scream = scream::parse_str((), "aaaaaaaaaaaaaaaaa").expect("couldn't parse");
    println!("{scream:?}");
}
