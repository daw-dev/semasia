use static_sdd::grammar;

pub trait Production {
    type Driver;
    type Body;

    fn synthesize(body: Self::Body) -> Self::Driver;
}

macro_rules! production {
    ($name:ident, $driver:ident -> $body:ty, |$param:pat_param| $clos:expr) => {
        #[doc = concat!("Production: `", stringify!($driver), " -> ", stringify!($body), "`")]
        pub struct $name;

        impl Production for $name {
            type Driver = $driver;
            type Body = $body;

            fn synthesize($param: Self::Body) -> Self::Driver {
                $clos
            }
        }
    };
}

#[grammar]
mod addition_grammar {
    use crate::Production;

    #[non_terminal]
    #[start_symbol]
    pub type E = f32;
    
    #[non_terminal]
    // #[start_symbol]
    pub type T = f32;

    #[token = r"\d+(\.\d*)?"]
    pub type Id = f32;

    #[token = "+"]
    pub struct Plus;

    production!(P1, E -> (E, Plus, T), |(e, _, t)| e + t);

    production!(P2, E -> T, |t| t);

    production!(P3, T -> Id, |id| id);
}

fn main() {
    parse();
}
