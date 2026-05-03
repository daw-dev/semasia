pub mod ast;
pub mod ctx;
pub mod expressions;
pub mod tokens;
pub mod types;

use semasia::grammar;

#[grammar]
#[logos(skip r"\s+")]
#[logos(skip r"\/\/.*")]
#[logos(skip r"\/\*(?m).*\*\/")]
pub mod language {
    use super::*;
    use semasia::*;

    #[context]
    use super::ctx::CompilationContext;

    #[start_symbol]
    #[non_terminal]
    pub use ast::Program;

    #[non_terminal]
    pub use ast::Statement;

    #[non_terminal]
    pub use expressions::Expression;

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

    #[non_terminal]
    #[auto_productions]
    #[derive(Debug, Clone)]
    pub enum Lit {
        Int(LitInt),
        Decimal(LitDecimal),
        Char(LitChar),
        String(LitString),
    }

    #[token("=")]
    #[priority(14)]
    pub struct Equals;

    #[token("==")]
    #[left_associative]
    #[priority(7)]
    pub struct EqualsEquals;

    #[token(">")]
    #[left_associative]
    #[priority(6)]
    pub struct GreaterThan;

    #[token("<")]
    #[left_associative]
    #[priority(6)]
    pub struct LessThan;

    #[token(";")]
    pub struct SemiColumn;

    #[token(",")]
    #[priority(15)]
    pub struct Comma;

    #[token("(")]
    pub struct OpenPar;

    #[token(")")]
    pub struct ClosePar;

    #[token("[")]
    #[left_associative]
    #[priority(1)]
    pub struct OpenSquare;

    #[token("]")]
    pub struct CloseSquare;

    #[token("{")]
    pub struct OpenCurly;

    #[token("}")]
    pub struct CloseCurly;

    #[token("+")]
    #[left_associative]
    #[priority(3)]
    pub struct Plus;

    #[token("*")]
    #[left_associative]
    #[priority(4)]
    pub struct Times;

    #[auto_productions]
    #[non_terminal]
    #[derive(Debug, Clone)]
    pub enum BinaryOperation {
        Sum(Box<Expression>, #[hide] Plus, Box<Expression>),
        Product(Box<Expression>, #[hide] Times, Box<Expression>),
        LessThan(Box<Expression>, #[hide] LessThan, Box<Expression>),
        GreaterThan(Box<Expression>, #[hide] GreaterThan, Box<Expression>),
        EqualityCheck(Box<Expression>, #[hide] EqualsEquals, Box<Expression>),
    }

    #[token("&")]
    #[right_associative]
    #[priority(3)]
    pub struct And;

    #[token("if")]
    pub struct If;

    #[token("else")]
    #[right_associative]
    pub struct Else;

    #[token("while")]
    pub struct While;

    #[token("for")]
    pub struct For;

    #[token("do")]
    pub struct Do;

    #[token("return")]
    pub struct Return;

    #[token("struct")]
    pub struct Struct;

    #[token("int")]
    pub struct Int;

    #[token("float")]
    pub struct Float;

    #[token("char")]
    pub struct Char;

    #[token("void")]
    pub struct Void;

    #[non_terminal]
    #[auto_productions]
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum BaseType {
        Int(#[hide] Int),
        Float(#[hide] Float),
        Char(#[hide] Char),
        Void(#[hide] Void),
        Ident(Ident),
    }

    #[non_terminal]
    #[auto_productions]
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum NonArrayType {
        Pointer(#[hide] Times, Box<NonArrayType>),
        BaseType(BaseType),
    }

    #[non_terminal]
    pub use types::TypedIdent;

    ebnf!(ProgramIsItems: Program -> Vec<Item>, |st| Program { root_items: st });

    #[non_terminal]
    pub use ast::StructDefinition;

    // STRUCTS
    ebnf!(StructCreation:
        StructDefinition -> (Struct, Ident, OpenCurly, Vec<(TypedIdent, SemiColumn)>, CloseCurly, SemiColumn),
        |(_, ident, _, fields, _, _)| {
            StructDefinition {
                ident,
                ty: types::StructType {
                    fields: fields.into_iter().map(|(ty_id, _)| (ty_id.ident, ty_id.ty)).collect()
                },
            }
        }
    );

    // TYPED IDENTS
    production!(TypedIdentIsNonArray: TypedIdent -> (NonArrayType, Ident), |(ty, ident)| TypedIdent { ty: ty.into(), ident });
    ebnf!(TypedIdentIsArray: TypedIdent -> (TypedIdent, OpenSquare, Option<LitInt>, CloseSquare),
        |(ty, _, _size, _)| {
            TypedIdent {
                ty: Type::Array(Box::new(ty.ty)),
                ident: ty.ident,
            }
        }
    );

    // ITEMS
    ebnf!(
        ItemIsFunction:
        Item ->
            (NonArrayType, Ident, OpenPar, #[separator(Comma)] Vec<TypedIdent>, ClosePar, OpenCurly, Vec<Statement>, CloseCurly),
        |ctx, (return_type, ident, _, params, _, _, body, _)| {
            Item::Function(ast::Function {
                return_type: return_type.into(),
                ident,
                params,
                body,
            })
        }
    );
    production!(ItemIsStruct: Item -> StructDefinition, |ty| Item::StructDefinition(ty));

    // EXPRESSIONS
    production!(ExpressionIsIdent: Expression -> Ident, |id| Expression::Ident(id));
    production!(ExpressionIsLit: Expression -> Lit, |lit| Expression::Lit(lit));
    production!(ExpressionIsDeref: Expression -> (Times, Expression), |(_, expr)| Expression::Deref(Box::new(expr)));
    production!(ExpressionIsParen: Expression -> (OpenPar, Expression, ClosePar), |(_, expr, _)| expr);
    ebnf!(
        ExpressionIsFunctionCall:
        Expression ->
            (Ident, OpenPar, #[separator(Comma)] Vec<Expression>, ClosePar),
        |(function_ident, _, arguments, _)| {
            Expression::FunctionCall(ast::FunctionCall { function_ident, arguments })
        }
    );
    production!(ExpressionIsIndexing: Expression -> (Expression, OpenSquare, Expression, CloseSquare), |(base, _, index, _)| Expression::Index(Box::new(base), Box::new(index)));
    production!(ExpressionIsReference: Expression -> (And, Expression), |(_, expr)| Expression::Reference(Box::new(expr)));
    production!(ExpressionIsBinOp: Expression -> BinaryOperation, |op| Expression::BinaryOperation(op));

    // STATEMENTS
    ebnf!(StatementIsBody: Statement -> (OpenCurly, Vec<Statement>, CloseCurly), |(_, statements, _)| {
        Statement::Braces(statements)
    });
    production!(Assignment: Statement -> (Ident, Equals, Expression, SemiColumn), |ctx, (ident, _, expr, _)| {
        match ctx.get_type(&ident) {
            Some(ty) => {
                let expr_type = expr.get_type(ctx);
                if !ty.compatible_with(&expr_type) {
                    panic!("cannot convert from {expr_type:?} to {ty:?}")
                }
            },
            None => {},
        }
        Statement::Assignment(ident, expr)
    });
    ebnf!(DeclarationStatement: Statement -> (TypedIdent, Option<(Equals, Expression)>, SemiColumn),
        |ctx, (TypedIdent { ty, ident }, val_opt, _)| {
            ctx.declare(ident.clone(), ty.clone());
            match val_opt {
                Some((_, val)) => {
                    Statement::Initialization(ty, ident, val)
                }
                None => {
                    Statement::Declaration(ty, ident)
                }
            }
        }
    );
    ebnf!(ReturnStatement: Statement -> (Return, Option<Expression>, SemiColumn), |(_, expr, _)| Statement::Return(expr));
    ebnf!(
        IfStatement:
        Statement ->
            (If, OpenPar, Expression, ClosePar, Statement, Option<(Else, Statement)>),
        |(_, _, condition, _, statement, else_st)| {
            Statement::IfStatement(
                condition,
                Box::new(statement),
                else_st.map(|(_, else_st)| Box::new(else_st)),
            )
        }
    );
    production!(
        WhileStatement:
        Statement -> (While, OpenPar, Expression, ClosePar, Statement),
        |(_, _, condition, _, stmt)| {
            Statement::WhileStatement(condition, Box::new(stmt))
        }
    );
    ebnf!(
        ForStatement:
        Statement -> (For, OpenPar, Option<Expression>, SemiColumn, Option<Expression>, SemiColumn, Option<Expression>, ClosePar, Statement),
        |(_, _, init, _, condition, _, step, _, stmt)| {
            Statement::ForStatement(init, condition, step, Box::new(stmt))
        }
    );
    production!(StatementIsExpression: Statement -> (Expression, SemiColumn), |(expr, _)| Statement::Expression(expr));
}
