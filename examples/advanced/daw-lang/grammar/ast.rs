use itertools::Itertools;

use crate::grammar::{expressions::Expression, tokens::Ident, types::Type};
use std::fmt::Display;

pub struct Indented<T>(pub T, pub usize);

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
    Body(Vec<Statement>),
}

impl Display for Indented<&Statement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "    ".repeat(self.1))?;
        match self.0 {
            Statement::Declaration(ty, id) => write!(f, "Declaration: {ty} {id};"),
            Statement::Initialization(ty, id, expr) => {
                write!(f, "Initialization: {ty} {id} = {expr};")
            }
            Statement::Assignment(id, expr) => write!(f, "Assignment: {id} = {expr};"),
            Statement::Expression(expr) => write!(f, "Expression: {expr};"),
            Statement::Return(expr) => {
                write!(f, "Return: ")?;
                match expr {
                    Some(expr) => write!(f, "return {expr};"),
                    None => write!(f, "return;"),
                }
            },
            Statement::Break => write!(f, "Break"),
            Statement::Continue => write!(f, "Continue"),
            Statement::Body(statements) => {
                writeln!(f, "{{")?;
                for stmt in statements.iter() {
                    writeln!(f, "{}", Indented(stmt, self.1 + 1))?;
                }
                write!(f, "}}")
            }
        }
    }
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

impl Display for Indented<&Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "    ".repeat(self.1))?;
        match self.0 {
            Item::Function(function) => {
                writeln!(
                    f,
                    "Function: {} {}({}) {{",
                    function.return_type,
                    function.ident,
                    function
                        .params
                        .iter()
                        .map(|(ty, id)| format!("{ty} {id}"))
                        .format(",")
                )?;
                for stmt in function.body.iter() {
                    writeln!(f, "{}", Indented(stmt, self.1 + 1))?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub root_items: Vec<Item>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program:")?;
        for item in self.root_items.iter() {
            writeln!(f, "{}", Indented(item, 0))?;
        }
        Ok(())
    }
}
