use std::fmt::Display;

use itertools::Itertools;

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
    Index(Box<Expression>, usize),
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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::LitInt(str)
            | Expression::LitDecimal(str)
            | Expression::LitChar(str)
            | Expression::LitString(str)
            | Expression::Ident(str) => write!(f, "{str}"),
            Expression::Deref(expr) => write!(f, "*{expr}"),
            Expression::Reference(expr) => write!(f, "&{expr}"),
            Expression::Index(expr, idx) => write!(f, "{expr}[{idx}]"),
            Expression::FunctionCall(func) => write!(
                f,
                "{}({})",
                func.function_ident,
                func.arguments.iter().format(", ")
            ),
            Expression::BinaryOperation(left, op, right) => write!(f, "{left} {op} {right}"),
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
            Operator::Times => write!(f, "-"),
            Operator::EqualsEquals => write!(f, "=="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessThan => write!(f, "<"),
        }
    }
}
