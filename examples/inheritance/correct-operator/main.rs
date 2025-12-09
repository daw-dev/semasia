use static_sdd::*;

#[grammar]
mod division {
    use super::*;

    pub enum Number {
        Integer(isize),
        Decimal(f32),
    }

    #[start_symbol]
    #[non_terminal]
    pub struct Expr {
        use_decimal: Inherited<bool>,
        result: Deferred<Number>,
    }

    #[token = r"\d+"]
    pub type Integer = isize;

    #[token = r"\d+\.\d+"]
    pub type Decimal = f32;

    #[token = r"\*"]
    pub struct Times;

    #[token = r"/"]
    pub struct Division;

    production!(P0, Expr -> (Expr, Times, Expr), |(e1, _, e2)| E {
        use_decimal: Inherited::inherit_multiple([e1.use_decimal, e2.use_decimal])
    });
}

fn main() {}
