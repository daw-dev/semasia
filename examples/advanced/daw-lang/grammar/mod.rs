pub mod ast;
pub mod ctx;
pub mod expressions;
pub mod tokens;
pub mod types;

use semasia::grammar;

#[grammar]
#[logos(skip r"\s+")]
#[logos(skip r"\/\/.*")]
#[logos(skip r"\/\*.*\*\/")]
pub mod language {
    use super::*;
    use grammar::left_associative;
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
    pub use expressions::Expression;

    #[non_terminal]
    pub type Arguments = Vec<Expression>;

    #[non_terminal]
    pub use ast::Item;

    #[non_terminal]
    pub use types::Type;

    #[regex(r"[a-zA-Z]\w*", to_string)]
    pub use tokens::Ident;

    #[regex(r"\d+", to_string)]
    pub use tokens::LitInt;

    #[regex(r"\d+\.\d+", to_string)]
    pub use tokens::LitDecimal;

    #[regex(r"'.'", to_string)]
    pub use tokens::LitChar;

    #[regex("\"[^\"]*\"", to_string)]
    pub use tokens::LitString;

    #[token("=")]
    pub struct Equals;

    #[token(";")]
    pub struct SemiColumn;

    #[token(",")]
    pub struct Comma;

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

    #[token("+")]
    #[left_associative]
    pub struct Plus;

    #[token("*")]
    #[left_associative]
    pub struct Times;

    #[non_terminal]
    pub use expressions::Operator;

    #[token("if")]
    pub struct If;

    #[token("else")]
    pub struct Else;

    #[token("while")]
    pub struct While;

    #[token("for")]
    pub struct For;

    #[token("do")]
    pub struct Do;

    #[token("return")]
    pub struct Return;

    ebnf!(ProgramIsItems, Program -> (Item*), |st| Program { root_items: st });

    // ITEMS
    ebnf!(
        ItemIsFunction,
        Item ->
            (Ident, Ident, OpenPar, (Type, Ident)*, ClosePar, OpenCurly, Statement*, CloseCurly),
        |ctx, (return_type, ident, _, params, _, _, body, _)| {
            ctx.declare(
                ident.clone(),
                Type::Function(
                    Box::new(Type::BaseType(return_type.clone())),
                    params
                        .iter()
                        .map(|(ty, _)| ty.clone())
                        .collect()));
            Item::Function(ast::Function {
                return_type: Type::BaseType(return_type),
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
    production!(ExpressionIsSum, Expression -> (Expression, Operator, Expression), |(left, op, right)| Expression::BinaryOperation(Box::new(left), op, Box::new(right)));
    production!(PlusOp, Operator -> Plus, |_| Operator::Plus);
    production!(TimesOp, Operator -> Times, |_| Operator::Times);
    production!(
        ExpressionIsFunctionCall,
        Expression ->
            (Ident, OpenPar, Arguments, ClosePar),
        |(function_ident, _, arguments, _)| {
            Expression::FunctionCall(ast::FunctionCall { function_ident, arguments })
        }
    );

    production!(ArgumentsIsDone, Arguments -> Expression, |e| vec![e]);
    production!(ArgumentsIsMore, Arguments -> (Arguments, Comma, Expression), |(mut t, _, expr)| {
        t.push(expr);
        t
    });

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
    ebnf!(Declaration, Statement -> (Ident, Ident, (Equals, Expression)?, SemiColumn), |ctx, (ty, id, val_opt, _)| {
        ctx.declare(id.clone(), Type::BaseType(ty.clone()));
        match val_opt {
            Some((_, val)) => {
                Statement::Initialization(Type::BaseType(ty), id, val)
            }
            None => {
                Statement::Declaration(Type::BaseType(ty), id)
            }
        }
    });
    ebnf!(ReturnStatement, Statement -> (Return, Expression?, SemiColumn), |(_, expr, _)| Statement::Return(expr));
    ebnf!(
        IfStatementBody,
        Statement ->
            (If, OpenPar, Expression, ClosePar, OpenCurly, Statement*, CloseCurly),
        |(_, _, condition, _, _, statements, _)| {
            todo!()
        }
    );
    production!(
        IfStatementOne,
        Statement ->
            (If, OpenPar, Expression, ClosePar, OpenCurly, Statement, CloseCurly),
        |(_, _, condition, _, _, statement, _)| {
            todo!()
        }
    );

    production!(StatementIsExpression, Statement -> (Expression, SemiColumn), |(expr, _)| Statement::Expression(expr));
}
