use static_sdd::*;

#[grammar]
mod compiler {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    pub enum Statement {
        Label(String),
        GoTo(String),
        BinOp(String, String, String, String),
        UnOp(String, String, String),
    }

    #[non_terminal]
    #[start_symbol]
    pub type P = Code;

    #[derive(Clone)]
    pub struct SNext {
        label: String,
    }

    pub struct Code {
        lines: Vec<Statement>,
    }

    #[non_terminal]
    pub type S = Pipe<SNext, Code>;

    pub struct BLabels {
        t: String,
        f: String,
    }

    #[non_terminal]
    pub type B = Pipe<BLabels, Code>;

    #[token = "skip"]
    pub struct Skip;

    #[token = "true"]
    pub struct True;

    #[token = "false"]
    pub struct False;

    #[token = "||"]
    pub struct OrOp;

    #[token = "&&"]
    pub struct AndOp;

    #[token = "if"]
    pub struct If;

    fn new_label() -> String {
        "L0".into()
    }

    production!(P0, P -> S, |s| s.supply(SNext { label: new_label() }));

    production!(P1, S -> (S, S), |(s1, s2)|
        s2.map_out(|code| {
            let mut res = s1.supply(SNext { label: new_label() });
            res.lines.extend(code.lines);
            res
        })
    );

    production!(P2, S -> (If, B, S), |(_, b, s)| {
        s.passthrough().map_out(move |(s_next, s_code)| {
            let mut res = b.supply(BLabels {
                    t: new_label(), f: s_next.label
                });
            res.lines.extend(s_code.lines);
            res
        })
    });
}

fn main() {
    compiler::parse("hello")
}
