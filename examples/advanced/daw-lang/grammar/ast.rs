use crate::grammar::{expressions::Expression, tokens::Ident, types::Type};

#[derive(Debug)]
pub struct FunctionCall {
    pub function_ident: Ident,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Type, Ident),
    Initialization(Type, Ident, Expression),
    Assignment(Ident, Expression),
    Expression(Expression),
    Return(Option<Expression>),
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Function {
    pub return_type: Type,
    pub ident: Ident,
    pub params: Vec<(Type, Ident)>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Item {
    Function(Function),
}
