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
            Expression::BinaryOperation(left, op, right) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Plus,
    Times,
}
