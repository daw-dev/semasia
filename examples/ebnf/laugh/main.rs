use semasia::*;

#[grammar]
mod laugh {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub struct Laugh;

    #[derive(Debug)]
    #[token("a")]
    #[token("A")]
    pub struct A;

    #[token("h")]
    #[token("H")]
    pub struct H;

    ebnf!(LaughProd: Laugh -> A * H, |_as| todo!());
}

fn main() {}
