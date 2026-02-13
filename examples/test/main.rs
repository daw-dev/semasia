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
    pub type __P0_0_Rep = Vec<A>;

    #[non_terminal]
    pub type __P0_1_Opt = Option<B>;

    #[non_terminal]
    pub enum CorD {
        C(C),
        D(D),
    }

    production!(__P0_0_More, __P0_0_Rep -> (__P0_0_Rep, A), |(mut acc, t)| { acc.push(t); acc });
    production!(__P0_0_Done, __P0_0_Rep -> (), |_| Vec::new());
    production!(__P0_1_Some, __P0_1_Opt -> B, |t| Some(t));
    production!(__P0_1_None, __P0_1_Opt -> (), |_| None);
    production!(__P0_2_CorD_C, CorD -> C, |t| CorD::C(t));
    production!(__P0_2_CorD_D, CorD -> D, |t| CorD::D(t));
    production!(P0, S -> (__P0_0_Rep, __P0_1_Opt, CorD), |(a, b, c_or_d)| S(a, b, match c_or_d { CorD :: C(_) => 0, CorD :: D(_) => 1 }));
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("aaaaad");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
