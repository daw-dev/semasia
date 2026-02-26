use static_sdd::*;

#[grammar]
mod division {
    use super::*;

    #[derive(PartialEq, Debug)]
    pub enum Number {
        Integer(isize),
        Decimal(f32),
    }

    impl Number {
        fn cast_to_decimal(self) -> Self {
            match self {
                Number::Integer(a) => Number::Decimal(a as f32),
                dec => dec,
            }
        }

        fn assert_integer(self) -> Self {
            match self {
                Number::Decimal(_) => panic!(),
                int => int,
            }
        }

        fn is_decimal(&self) -> bool {
            match self {
                Number::Decimal(_) => true,
                Number::Integer(_) => false,
            }
        }

        fn unwrap_integer(self) -> isize {
            match self {
                Number::Integer(a) => a,
                _ => panic!(),
            }
        }

        fn unwrap_decimal(self) -> f32 {
            match self {
                Number::Decimal(a) => a,
                _ => panic!(),
            }
        }
    }

    #[start_symbol]
    #[non_terminal]
    pub type S = Number;

    #[non_terminal]
    pub struct Expr {
        use_decimal: bool,
        decimal: FromInherited<bool, Number>,
    }

    #[non_terminal]
    pub type Term = Number;

    #[token(regex = r"\d+")]
    pub type Integer = isize;

    #[token(regex = r"\d+\.\d+")]
    pub type Decimal = f32;

    #[token("*")]
    pub struct Times;

    #[token("/")]
    pub struct Division;

    production!(P0, S -> Expr, |e| e.decimal.resolve(e.use_decimal));

    production!(P1, Expr -> (Expr, Times, Term), |(e, _, t)| {
        Expr {
            use_decimal: e.use_decimal || t.is_decimal(),
            decimal: e.decimal.synthesize(|use_decimal, e_result| {
                if use_decimal {
                    Number::Decimal(e_result.unwrap_decimal() * t.unwrap_decimal())
                } else {
                    Number::Integer(e_result.unwrap_integer() * t.unwrap_integer())
                }
            })
        }
    });

    production!(P11, Expr -> (Expr, Division, Term), |(e, _, t)| {
        Expr {
            use_decimal: e.use_decimal || t.is_decimal(),
            decimal: e.decimal.synthesize(|use_decimal, e_result| {
                let t = if use_decimal {
                    t.cast_to_decimal()
                } else {
                    t.assert_integer()
                };
                if use_decimal {
                    Number::Decimal(e_result.unwrap_decimal() / t.unwrap_decimal())
                } else {
                    Number::Integer(e_result.unwrap_integer() / t.unwrap_integer())
                }
            })
        }
    });

    production!(P2, Expr -> Term, |t| Expr {
        use_decimal: t.is_decimal(),
        decimal: FromInherited::new(|use_decimal|
            if use_decimal {
                t.cast_to_decimal()
            } else {
                t.assert_integer()
            }
        )
    });

    production!(P3, Term -> Integer, |i| Number::Integer(i));
    production!(P4, Term -> Decimal, |f| Number::Decimal(f));
}

fn main() {
    let res = division::parse_str((), "5/2*1.0").expect("couldn't parse");
    println!("{res:?}");
}
