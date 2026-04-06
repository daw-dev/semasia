use std::fmt::Display;

use itertools::Itertools;

use crate::grammar::ast::*;

pub struct Indented<T>(pub T, pub usize);

impl Display for Indented<&Statement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Indented(stmt, indentation) = self;
        write!(f, "{}", "    ".repeat(*indentation))?;
        match stmt {
            Statement::Declaration(ty, id) => {
                writeln!(f, "Declaration:")?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Type: {ty}")?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Ident: {id}")
            }
            Statement::Initialization(ty, id, expr) => {
                writeln!(f, "Declaration:")?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Type: {ty}")?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Ident: {id}")?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Value: {expr}")
            }
            Statement::Assignment(id, expr) => {
                writeln!(f, "Assignment:")?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Destination: {id}")?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Value: {expr}")
            }
            Statement::Expression(expr) => write!(f, "Expression: {expr};"),
            Statement::Return(expr) => {
                write!(f, "Return:")?;
                if let Some(expr) = expr {
                    writeln!(f)?;
                    write!(f, "{}  ", "    ".repeat(*indentation))?;
                    write!(f, "Expression: {expr}")?;
                }
                Ok(())
            }
            Statement::Break => write!(f, "Break"),
            Statement::Continue => write!(f, "Continue"),
            Statement::Body(statements) => {
                write!(f, "Body:")?;
                for stmt in statements.iter() {
                    writeln!(f)?;
                    write!(f, "{}", Indented(stmt, indentation + 1))?;
                }
                Ok(())
            }
            Statement::IfStatement(condition, body, else_st) => {
                writeln!(f, "IfStatement:")?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Condition: {condition}")?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                writeln!(
                    f,
                    "Statement:",
                )?;

                write!(f, "{}", Indented(body.as_ref(), indentation + 1))?;

                if let Some(else_st) = else_st {
                    writeln!(f)?;

                    write!(f, "{}  ", "    ".repeat(*indentation))?;
                    writeln!(
                        f,
                        "Else Statement:",
                    )?;
                    write!(f, "{}", Indented(else_st.as_ref(), indentation + 1))?;
                }

                Ok(())
            }
            Statement::WhileStatement(condition, body) => {
                writeln!(f, "WhileStatement:")?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Condition: {condition}")?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                writeln!(
                    f,
                    "Statement:",
                )?;

                write!(f, "{}", Indented(body.as_ref(), indentation + 1))?;

                Ok(())
            }
        }
    }
}

impl Display for Indented<&Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Indented(item, indentation) = self;
        write!(f, "{}", "    ".repeat(*indentation))?;
        match item {
            Item::Function(function) => {
                writeln!(f, "Function:")?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Return type: {}", function.return_type)?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Ident: {}", function.ident)?;

                writeln!(f)?;

                write!(f, "{}  ", "    ".repeat(*indentation))?;
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

                write!(f, "{}  ", "    ".repeat(*indentation))?;
                write!(f, "Body:")?;

                for stmt in function.body.iter() {
                    writeln!(f)?;
                    write!(f, "{}", Indented(stmt, self.1 + 1))?;
                }
                Ok(())
            }
        }
    }
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
