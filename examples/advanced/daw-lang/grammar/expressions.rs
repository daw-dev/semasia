use std::fmt::Display;

use crate::grammar::{ast::FunctionCall, ctx::CompilationContext, tokens::*, types::Type};

#[derive(Debug)]
pub enum Expression {
    LitInt(LitInt),
    LitDecimal(LitDecimal),
    LitChar(LitChar),
    LitString(LitString),
    Ident(Ident),
    Deref(Box<Expression>),
    Reference(Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
    FunctionCall(FunctionCall),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}

impl Expression {
    pub fn get_type(&self, ctx: &CompilationContext) -> Type {
        match self {
            Expression::LitInt(_) => Type::int(),
            Expression::LitDecimal(_) => Type::float(),
            Expression::LitChar(_) => Type::char(),
            Expression::LitString(_) => Type::string(),
            Expression::Ident(id) => ctx
                .get_type(id)
                .expect("ident refers to a undeclared variable"),
            Expression::Deref(expr) => expr.get_type(ctx).deref(),
            Expression::Reference(expr) => Type::Pointer(Box::new(expr.get_type(ctx))),
            Expression::Index(expr, _) => expr.get_type(ctx).deref(),
            Expression::FunctionCall(func) => ctx
                .get_type(&func.function_ident)
                .expect("such function does not exist"),
            Expression::BinaryOperation(left, _op, _right) => left.get_type(ctx),
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
