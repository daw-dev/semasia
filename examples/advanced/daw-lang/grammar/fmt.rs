use std::{cell::RefCell, fmt::Display, rc::Rc};

use itertools::Itertools;

use crate::grammar::ast::*;

pub enum IndentationType {
    Space,
    Middle,
    Last,
}

pub struct Indented<T>(pub T, pub Rc<RefCell<Vec<IndentationType>>>);

impl Display for Indented<&Statement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Indented(stmt, indentation) = self;
        let indentation_str = indentation
            .borrow()
            .iter()
            .map(|ty| match ty {
                IndentationType::Middle => " ┃ ",
                IndentationType::Space => "   ",
                IndentationType::Last => "   ",
            })
            .format("")
            .to_string();
        write!(
            f,
            "{}",
            indentation
                .borrow()
                .iter()
                .enumerate()
                .map(|(i, ty)| match ty {
                    IndentationType::Middle if i == indentation.borrow().len() - 1 => " ┣━",
                    IndentationType::Middle => " ┃ ",
                    IndentationType::Space => "   ",
                    IndentationType::Last if i == indentation.borrow().len() - 1 => " ┗━",
                    IndentationType::Last => "   ",
                })
                .format("")
                .to_string()
        )?;
        match stmt {
            Statement::Declaration(ty, id) => {
                writeln!(f, "Declaration:")?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Type: {ty}")?;

                writeln!(f)?;

                write!(f, "{} ┗━", indentation_str)?;
                write!(f, "Ident: {id}")
            }
            Statement::Initialization(ty, id, expr) => {
                writeln!(f, "Declaration:")?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Type: {ty}")?;

                writeln!(f)?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Ident: {id}")?;

                writeln!(f)?;

                write!(f, "{} ┗━", indentation_str)?;
                write!(f, "Value: {expr}")
            }
            Statement::Assignment(id, expr) => {
                writeln!(f, "Assignment:")?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Destination: {id}")?;

                writeln!(f)?;

                write!(f, "{} ┗━", indentation_str)?;
                write!(f, "Value: {expr}")
            }
            Statement::Expression(expr) => write!(f, "Expression: {expr};"),
            Statement::Return(expr) => {
                write!(f, "Return:")?;
                if let Some(expr) = expr {
                    writeln!(f)?;
                    write!(f, "{} ┗━", indentation_str)?;
                    write!(f, "Expression: {expr}")?;
                }
                Ok(())
            }
            Statement::Break => write!(f, "Break"),
            Statement::Continue => write!(f, "Continue"),
            Statement::Body(statements) => {
                write!(f, "Body:")?;
                for (i, stmt) in statements.iter().enumerate() {
                    writeln!(f)?;
                    if i == statements.len() - 1 {
                        indentation.borrow_mut().push(IndentationType::Last);
                    } else {
                        indentation.borrow_mut().push(IndentationType::Middle);
                    }
                    write!(f, "{}", Indented(stmt, indentation.clone()))?;
                    indentation.borrow_mut().pop();
                }
                Ok(())
            }
            Statement::IfStatement(condition, body, else_st) => {
                writeln!(f, "IfStatement:")?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Condition: {condition}")?;

                writeln!(f)?;

                if let Some(else_st) = else_st {
                    write!(f, "{} ┣━", indentation_str)?;
                    writeln!(f, "TrueStatement:")?;

                    indentation.borrow_mut().push(IndentationType::Middle);
                    indentation.borrow_mut().push(IndentationType::Last);

                    write!(f, "{}", Indented(body.as_ref(), indentation.clone()))?;

                    indentation.borrow_mut().pop();
                    indentation.borrow_mut().pop();

                    writeln!(f)?;

                    write!(f, "{} ┗━", indentation_str)?;
                    writeln!(f, "FalseStatement:",)?;

                    indentation.borrow_mut().push(IndentationType::Space);
                    indentation.borrow_mut().push(IndentationType::Last);

                    write!(f, "{}", Indented(else_st.as_ref(), indentation.clone()))?;

                    indentation.borrow_mut().pop();
                    indentation.borrow_mut().pop();
                } else {
                    write!(f, "{} ┗━", indentation_str)?;
                    writeln!(f, "TrueStatement:")?;

                    indentation.borrow_mut().push(IndentationType::Space);
                    indentation.borrow_mut().push(IndentationType::Last);

                    write!(f, "{}", Indented(body.as_ref(), indentation.clone()))?;

                    indentation.borrow_mut().pop();
                    indentation.borrow_mut().pop();
                }

                Ok(())
            }
            Statement::WhileStatement(condition, body) => {
                writeln!(f, "WhileStatement:")?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Condition: {condition}")?;

                writeln!(f)?;

                write!(f, "{} ┗━", indentation_str)?;
                writeln!(f, "Statement:",)?;

                indentation.borrow_mut().push(IndentationType::Space);
                indentation.borrow_mut().push(IndentationType::Last);

                write!(f, "{}", Indented(body.as_ref(), indentation.clone()))?;

                indentation.borrow_mut().pop();
                indentation.borrow_mut().pop();

                Ok(())
            }
        }
    }
}

impl Display for Indented<&Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Indented(item, indentation) = self;
        let indentation_str = indentation
            .borrow()
            .iter()
            .map(|ty| match ty {
                IndentationType::Middle => " ┃ ",
                IndentationType::Space => "   ",
                IndentationType::Last => "   ",
            })
            .format("")
            .to_string();
        write!(
            f,
            "{}",
            indentation
                .borrow()
                .iter()
                .enumerate()
                .map(|(i, ty)| match ty {
                    IndentationType::Middle if i == indentation.borrow().len() - 1 => " ┣━",
                    IndentationType::Middle => " ┃ ",
                    IndentationType::Space => "   ",
                    IndentationType::Last if i == indentation.borrow().len() - 1 => " ┗━",
                    IndentationType::Last => "   ",
                })
                .format("")
                .to_string()
        )?;
        match item {
            Item::Function(function) => {
                writeln!(f, "Function:")?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Return type: {}", function.return_type)?;

                writeln!(f)?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(f, "Ident: {}", function.ident)?;

                writeln!(f)?;

                write!(f, "{} ┣━", indentation_str)?;
                write!(
                    f,
                    "Params: {}",
                    function
                        .params
                        .iter()
                        .map(|(ty, id)| format!("{ty} {id}"))
                        .format(",")
                )?;

                writeln!(f)?;

                write!(f, "{} ┗━", indentation_str)?;
                write!(f, "Body:")?;

                indentation.borrow_mut().push(IndentationType::Space);

                for (i, stmt) in function.body.iter().enumerate() {
                    if i == function.body.len() - 1 {
                        indentation.borrow_mut().push(IndentationType::Last);
                    } else {
                        indentation.borrow_mut().push(IndentationType::Middle);
                    }
                    writeln!(f)?;
                    write!(f, "{}", Indented(stmt, indentation.clone()))?;
                    indentation.borrow_mut().pop();
                }
                Ok(())
            }
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program:")?;
        for (i, item) in self.root_items.iter().enumerate() {
            writeln!(f)?;
            if i == self.root_items.len() - 1 {
                write!(f, "{}", Indented(item, Rc::new(RefCell::new(vec![IndentationType::Last]))))?;
            } else {
                write!(f, "{}", Indented(item, Rc::new(RefCell::new(vec![IndentationType::Middle]))))?;
            }
        }
        Ok(())
    }
}
