pub mod ast;
pub mod ctx;
pub mod tokens;
pub mod types;

use semasia::grammar;

#[grammar]
#[logos(skip r"[\s\t\f]+")]
pub mod language {
    use super::*;
    use semasia::*;

    #[context]
    use super::ctx::CompilationContext;

    #[start_symbol]
    #[non_terminal]
    #[derive(Debug)]
    pub struct Program {
        root_items: Vec<Item>,
    }

    #[non_terminal]
    pub use ast::Statement;

    #[non_terminal]
    pub use ast::Expression;

    #[non_terminal]
    pub use ast::Item;

    #[non_terminal]
    pub use types::Type;

    #[token(regex = r"[a-zA-Z]\w*")]
    pub use tokens::Ident;

    #[token(regex = r"\d+")]
    pub use tokens::LitInt;

    #[token(regex = r"\d+\.\d+")]
    pub use tokens::LitDecimal;

    #[token(regex = r"'.'")]
    pub use tokens::LitChar;

    #[token(regex = "\"[^\"]*\"")]
    pub use tokens::LitString;

    #[token("=")]
    pub struct Equals;

    #[token(";")]
    pub struct SemiColumn;

    #[token("return")]
    pub struct Return;

    #[token(regex = r"//\w*$")]
    pub type Comment = String;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosePar;

    #[token("[")]
    pub struct OpenSquare;

    #[token("]")]
    pub struct CloseSquare;

    #[token("{")]
    pub struct OpenCurly;

    #[token("}")]
    pub struct CloseCurly;

    ebnf!(ProgramIsStatement, Program -> (Item*), |st| Program { root_items: st });

    // ITEMS
    ebnf!(
        ItemIsFunction,
        Item ->
            (Type, Ident, OpenPar, (Type, Ident)*, ClosePar, OpenCurly, Statement*, CloseCurly),
        |(return_type, ident, _, params, _, _, body, _)| {
            Item::Function(ast::Function {
                return_type,
                ident,
                params,
                body,
            })
        }
    );

    // EXPRESSIONS
    production!(ExpressionIsIdent, Expression -> Ident, |id| Expression::Ident(id));
    production!(ExpressionIsLitInt, Expression -> LitInt, |lit| Expression::LitInt(lit));
    production!(ExpressionIsLitDecimal, Expression -> LitDecimal, |lit| Expression::LitDecimal(lit));

    // STATEMENTS
    production!(Assignment, Statement -> (Ident, Equals, Expression, SemiColumn), |ctx, (ident, _, expr, _)| {
        match ctx.get_type(&ident) {
            Some(ty) => {
                let expr_type = expr.get_type(ctx);
                if ty.compatible_with(&expr_type) {
                    panic!("cannot convert from {expr_type:?} to {ty:?}")
                }
            },
            None => {},
        }
        Statement::Assignment(ident, expr)
    });

    ebnf!(Declaration, Statement -> (Type, Ident, (Equals, Expression)?, SemiColumn), |ctx, (ty, id, val_opt, _)| {
        ctx.declare(&id, &ty);
        match val_opt {
            Some((_, val)) => {
                Statement::Initialization(ty, id, val)
            }
            None => {
                Statement::Declaration(ty, id)
            }
        }
    });

    ebnf!(ReturnStatement, Statement -> (Return, Expression?, SemiColumn), |(_, expr, _)| Statement::Return(expr));
}
