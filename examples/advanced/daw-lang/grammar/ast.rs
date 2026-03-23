use crate::grammar::{
    ctx::CompilationContext,
    tokens::{Ident, LitChar, LitDecimal, LitInt, LitString},
    types::Type,
};

#[derive(Debug)]
pub struct FunctionCall {
    function_ident: Ident,
    arguments: Vec<Expression>,
}

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
        }
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
