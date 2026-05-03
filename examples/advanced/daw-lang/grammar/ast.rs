use std::fmt::Display;

use itertools::Itertools;

use crate::grammar::{
    expressions::Expression,
    tokens::Ident,
    types::{StructType, Type, TypedIdent},
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function_ident: Ident,
    pub arguments: Vec<Expression>,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.function_ident,
            self.arguments.iter().format(", ")
        )
    }
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
    Braces(Vec<Statement>),
    IfStatement(Expression, Box<Statement>, Option<Box<Statement>>),
    WhileStatement(Expression, Box<Statement>),
    ForStatement(
        Option<Expression>,
        Option<Expression>,
        Option<Expression>,
        Box<Statement>,
    ),
}

#[derive(Debug)]
pub struct Function {
    pub return_type: Type,
    pub ident: Ident,
    pub params: Vec<TypedIdent>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub ident: Ident,
    pub ty: StructType,
}

#[derive(Debug)]
pub enum Item {
    Function(Function),
    StructDefinition(StructDefinition),
}

#[derive(Debug)]
pub struct Program {
    pub root_items: Vec<Item>,
}
