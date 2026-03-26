use semasia::*;

#[grammar]
#[logos(skip r"\s+")]
pub mod grammar {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    #[derive(Debug)]
    pub enum Function {
        Constant(Constant),
        Variable(Variable),
        Sum(Box<Function>, Box<Function>),
        Product(Box<Function>, Box<Function>),
        Negation(Box<Function>),
        Power(Box<Function>, Box<Function>),
        Logarithm(Box<Function>, Box<Function>),
    }

    #[regex(r"\d+(\.\d+)?", parse)]
    pub type Constant = f32;

    #[regex(r"[A-Za-z]([A-Za-z]|\d)*", to_string)]
    pub type Variable = String;

    #[token("+")]
    #[left_associative]
    pub struct Plus;

    #[token("*")]
    #[left_associative]
    pub struct Times;

    production!(FunctionIsConstant, Function -> Constant, |c| Function::Constant(c));
    production!(FunctionIsVariable, Function -> Variable, |var| Function::Variable(var));
    production!(FunctionIsSum, Function -> (Function, Plus, Function), |(f1, _, f2)| Function::Sum(Box::new(f1), Box::new(f2)));
    production!(FunctionIsProduct, Function -> (Function, Times, Function), |(f1, _, f2)| Function::Product(Box::new(f1), Box::new(f2)));
}
