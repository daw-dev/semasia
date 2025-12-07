use static_sdd::*;

#[grammar]
mod arrays {
    use super::*;

    pub enum ComputedType {
        BaseType(String),
        Array(usize, Box<ComputedType>),
    }

    #[non_terminal]
    #[start_symbol]
    pub type T = ComputedType;

    #[non_terminal]
    pub struct C {
        base_type: Inherited<String>,
        computed_type: Inherited<ComputedType>,
    }

    #[token = "int|float"]
    pub type B = String;

    #[token = "["]
    pub struct LeftSquarePar;

    #[token = "]"]
    pub struct RightSquarePar;

    #[token = r"\d+"]
    pub type Size = usize;

    production!(P1, T -> (B, C), |(b, c)| {
        c.base_type.set(b);
        c.computed_type.unwrap_consume()
    });

    production!(P2, C -> (LeftSquarePar, Size, RightSquarePar, C), |(_, size, _, c)| {
        C {
            base_type: Inherited::inherit(c.base_type),
            computed_type: c.computed_type.map(move |t| ComputedType::Array(size, Box::new(t))),
        }
    });

    production!(P3, C -> (), |_| {
        let computed_out = Inherited::new();

        C {
            base_type: Inherited::inherit_map(computed_out.clone(), ComputedType::BaseType),
            computed_type: computed_out,
        }
    });
}

fn main() {
    parse();
}
