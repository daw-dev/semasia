use semasia::*;

#[grammar]
#[logos(skip r"\s+")]
mod addition {
    use super::*;

    #[start_symbol]
    #[non_terminal]
    pub type Expression = usize;

    #[regex(r"\d+", parse)]
    pub type Number = usize;

    #[token("+")]
    pub struct Plus;

    ebnf!(Addition, Expression -> (Number, (Plus, Number)*), |(first, others)| {
        others.into_iter().map(|(_, num)| num).fold(first, |acc, curr| acc + curr)
    });
}

pub fn main() {
    let result = addition::Parser::lex_parse("1 + 2 + 3 + 4");
    match result {
        Ok(result) => {
            println!("{result}");
        }
        Err(err) => {
            println!("{err}");
        }
    }
}
