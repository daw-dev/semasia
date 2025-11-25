use static_sdd::*;

#[grammar]
mod ast {
    use super::*;

    enum ExprNode {
        Plus(Box<ExprNode>, Box<ExprNode>),
        Value(usize),
    }

    impl ExprNode {
        pub fn compute(self) -> usize {
            match self {
                ExprNode::Plus(left, right) => {
                    left.compute() + right.compute()
                }
                ExprNode::Value(v) => v,
            }
        }
    }

    #[non_terminal]
    #[start_symbol]
    type E = ExprNode;

    #[non_terminal]
    type T = ExprNode;

    #[token = r"\d+"]
    type Id = usize;

    #[token = r"\+"]
    struct Plus;

    production!(P1, E -> (E, Plus, T), |(e, _, t)| ExprNode::Plus(Box::new(e), Box::new(t)));

    production!(P2, E -> T, |t| t);

    production!(P3, T -> Id, |id| ExprNode::Value(id));
}

fn main() {
    parse();
}
