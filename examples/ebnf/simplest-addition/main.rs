use semasia::*;

#[grammar]
#[logos(skip r"\s+")]
mod addition {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub type Expression = usize;

    #[token("+")]
    pub struct Plus;

    #[regex(r"\d+", parse)]
    pub type Number = Expression;

    ebnf!(Addition, Expression -> (Number * Plus), |numbers| numbers.into_iter().sum());
}

fn main() {
    let result = addition::Parser::lex_parse("1 + 2 + 3");
    match result {
        Ok(result) => println!("result is {result}"),
        Err(err) => eprintln!("{err}"),
    }
}
