use semasia::*;

#[grammar]
#[logos(skip r"\s+")]
mod expressions {
    use super::*;
    use std::ops::{Add, Mul};

    #[start_symbol]
    #[non_terminal]
    #[derive(Debug)]
    pub enum Expression {
        Number(usize),
        Sum(Box<Expression>, Box<Expression>),
        Product(Box<Expression>, Box<Expression>),
    }

    impl Add for Expression {
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Expression::Sum(Box::new(self), Box::new(other))
        }
    }

    impl Mul for Expression {
        type Output = Self;
        fn mul(self, other: Self) -> Self::Output {
            Expression::Product(Box::new(self), Box::new(other))
        }
    }

    impl Expression {
        pub fn compute(self) -> usize {
            match self {
                Expression::Number(res) => res,
                Expression::Sum(left, right) => left.compute() + right.compute(),
                Expression::Product(left, right) => left.compute() * right.compute(),
            }
        }
    }

    #[non_terminal]
    pub type Term = Expression;

    #[non_terminal]
    pub type Factor = Expression;

    #[regex(r"\d+", parse)]
    pub type Number = usize;

    #[token("+")]
    pub struct Plus;

    #[token("*")]
    pub struct Times;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosedPar;

    production!(Addition, Expression -> (Expression, Plus, Term), |(e, _ ,t)| e + t);
    production!(NoAddition, Expression -> Term);
    production!(Multiplication, Term -> (Term, Times, Factor), |(t, _, f)| t * f);
    production!(NoMultiplication, Term -> Factor);
    production!(Parenthesis, Factor -> (OpenPar, Expression, ClosedPar), |(_, e, _)| e);
    production!(ActualNumber, Factor -> Number, |n| Expression::Number(n));
}

use expressions::*;

fn main() {
    let res = Parser::lex_parse("(1 + 2) * 3 + 4");

    match res {
        Ok(res) => {
            println!("abstract syntax tree is {res:?}");
            println!("result is {}", res.compute());
        },
        Err(err) => eprintln!("{err}"),
    }
}
