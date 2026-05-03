use std::fmt::Display;

use crate::grammar::{
    ast::FunctionCall,
    ctx::CompilationContext,
    language::{BaseType, BinaryOperation, Lit},
    tokens::*,
    types::Type,
};

impl Lit {
    pub fn get_type(&self) -> Type {
        match self {
            Lit::Int(_) => Type::Base(BaseType::Int),
            Lit::Decimal(_) => Type::Base(BaseType::Float),
            Lit::Char(_) => Type::Base(BaseType::Char),
            Lit::String(_) => Type::Pointer(Box::new(Type::Base(BaseType::Char))),
        }
    }
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Int(str) | Lit::Decimal(str) | Lit::Char(str) | Lit::String(str) => {
                write!(f, "{str}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Lit(Lit),
    Ident(Ident),
    Deref(Box<Expression>),
    Reference(Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
    FunctionCall(FunctionCall),
    BinaryOperation(BinaryOperation),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Lit(lit) => write!(f, "{lit}"),
            Expression::Ident(id) => write!(f, "{id}"),
            Expression::Deref(expr) => write!(f, "*{expr}"),
            Expression::Reference(expr) => write!(f, "&{expr}"),
            Expression::Index(base, index) => write!(f, "{base}[{index}]"),
            Expression::FunctionCall(func) => write!(f, "{func}"),
            Expression::BinaryOperation(binop) => write!(f, "{binop}"),
        }
    }
}

impl Expression {
    pub fn get_type(&self, ctx: &CompilationContext) -> Type {
        match self {
            Expression::Lit(lit) => lit.get_type(),
            Expression::Ident(id) => ctx
                .get_type(id)
                .cloned()
                .expect("ident refers to a undeclared variable"),
            Expression::Deref(expr) => expr.get_type(ctx).deref(),
            Expression::Reference(expr) => Type::Pointer(Box::new(expr.get_type(ctx))),
            Expression::Index(expr, _) => expr.get_type(ctx).deref(),
            Expression::FunctionCall(func) => ctx
                .get_type(&func.function_ident)
                .cloned()
                .expect("such function does not exist"),
            Expression::BinaryOperation(binop) => binop.get_type(ctx),
        }
    }
}

impl BinaryOperation {
    pub fn get_type(&self, ctx: &CompilationContext) -> Type {
        match self {
            BinaryOperation::Sum(left, _) | BinaryOperation::Product(left, _) => left.get_type(ctx),
            _ => Type::Base(BaseType::Int),
        }
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperation::Sum(left, right) => write!(f, "{left} + {right}"),
            BinaryOperation::Product(left, right) => write!(f, "{left} * {right}"),
            BinaryOperation::LessThan(left, right) => write!(f, "{left} < {right}"),
            BinaryOperation::GreaterThan(left, right) => write!(f, "{left} > {right}"),
            BinaryOperation::EqualityCheck(left, right) => write!(f, "{left} == {right}"),
        }
    }
}
