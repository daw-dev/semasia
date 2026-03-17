use semasia::*;

#[grammar]
mod addition {
    use super::*;

    #[start_symbol]
    #[non_terminal]
    pub type Expression = usize;

    #[token(regex = r"\d+")]
    pub type Number = usize;

    #[token("+")]
    pub struct Plus;

    ebnf!(Addition, Expression -> (Number, (Plus, Number)*), |(first, others)| {
        others.into_iter().map(|(_, num)| num).fold(first, |acc, curr| acc + curr)
    });
}

pub fn main() {

}
