use semasia::grammar;

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

    #[non_terminal]
    pub type APrime = Vec<A>;

    #[non_terminal]
    pub type BPrime = Option<B>;

    #[non_terminal]
    pub enum CorD {
        C(C),
        D(D),
    }

    production!(MoreAs, APrime -> (APrime, A), |(mut acc, t)| { acc.push(t); acc });
    production!(StopAs, APrime -> (), |_| Vec::new());
    production!(SomeB, BPrime -> B, |t| Some(t));
    production!(NoneB, BPrime -> (), |_| None);
    production!(CorDisC, CorD -> C, |t| CorD::C(t));
    production!(CorDisD, CorD -> D, |t| CorD::D(t));
    production!(P0, S -> (APrime, BPrime, CorD), |(a, b, c_or_d)| S(a, b, match c_or_d { CorD :: C(_) => 0, CorD :: D(_) => 1 }));
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("aaaaad");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
