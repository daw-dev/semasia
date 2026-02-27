#![semasia::grammar]

use semasia::*;

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

#[token(regex = r"\d+(\.\d+)?")]
pub type Constant = f32;

#[token(regex = r"[A-Za-z]([A-Za-z]|\d)*")]
pub type Variable = String;

#[token("+")]
pub struct Plus;

#[token("*")]
pub struct Times;

production!(FunctionIsConstant, Function -> Constant, |c| Function::Constant(c));
production!(FunctionIsVariable, Function -> Variable, |var| Function::Variable(var));
production!(FunctionIsSum, Function -> (Function, Plus, Function), |(f1, _, f2)| Function::Sum(Box::new(f1), Box::new(f2)));
production!(FunctionIsProduct, Function -> (Function, Times, Function), |(f1, _, f2)| Function::Product(Box::new(f1), Box::new(f2)));

