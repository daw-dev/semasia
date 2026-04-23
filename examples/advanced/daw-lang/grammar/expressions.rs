use std::fmt::Display;

use crate::grammar::{
    ast::FunctionCall, ctx::CompilationContext, language::{BinaryOperation, Lit}, tokens::*, types::Type,
};

impl Lit {
    pub fn get_type(&self) -> Type {
        match self {
            Lit::Int(_) => Type::int(),
            Lit::Decimal(_) => Type::float(),
            Lit::Char(_) => Type::char(),
            Lit::String(_) => Type::string(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Lit(Lit),
    Ident(Ident),
    Deref(Box<Expression>),
    Reference(Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
    FunctionCall(FunctionCall),
    BinaryOperation(BinaryOperation),
}

impl Expression {
    pub fn get_type(&self, ctx: &CompilationContext) -> Type {
        match self {
            Expression::Lit(lit) => lit.get_type(),
            Expression::Ident(id) => ctx
                .get_type(id)
                .expect("ident refers to a undeclared variable"),
            Expression::Deref(expr) => expr.get_type(ctx).deref(),
            Expression::Reference(expr) => Type::Pointer(Box::new(expr.get_type(ctx))),
            Expression::Index(expr, _) => expr.get_type(ctx).deref(),
            Expression::FunctionCall(func) => ctx
                .get_type(&func.function_ident)
                .expect("such function does not exist"),
            Expression::BinaryOperation(binop) => binop.get_type(ctx),
        }
    }
}

impl BinaryOperation {
    pub fn get_type(&self, ctx: &CompilationContext) -> Type {
        match self {
            BinaryOperation::Sum(left, _) | BinaryOperation::Product(left, _) => {
                left.get_type(ctx)
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Plus,
    Times,
    EqualsEquals,
    GreaterThan,
    LessThan,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Times => write!(f, "*"),
            Operator::EqualsEquals => write!(f, "=="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessThan => write!(f, "<"),
        }
    }
}
