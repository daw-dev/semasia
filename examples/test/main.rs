use semasia::grammar;

#[grammar]
#[logos(skip r"\s")]
mod abcs {
    use semasia::*;

    #[non_terminal]
    #[start_symbol]
    pub type Expr = usize;

    #[regex(r"\d+", parse)]
    pub type Num = usize;

    #[token("+")]
    pub struct Plus;

    production!(Sum: Expr -> (Expr, Plus, Num), |(left, _, right)| left + right);

    production!(Number: Expr -> Num);
}

use abcs::Parser;

fn main() {
    let res = Parser::lex_parse("5 + 3 128");
    match res {
        Ok(res) => println!("{res}"),
        Err(err) => eprintln!("{err}"),
    }
}
