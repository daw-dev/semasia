use ptree::{TreeBuilder, item::StringItem};

use crate::grammar::{
    ast::{Function, FunctionCall, Item, Program, Statement, StructDefinition},
    language::{BinaryOperation, Expression, Lit},
};

impl FunctionCall {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        tree.begin_child(String::from("Function Call:"));

        tree.add_empty_child(format!("Ident: {}", self.function_ident));

        tree.begin_child(String::from("Arguments"));

        for expr in self.arguments.iter() {
            expr.build_tree(tree);
        }

        tree.end_child();
        tree.end_child();
    }
}

impl BinaryOperation {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        match self {
            BinaryOperation::Sum(left, right) => {
                tree.begin_child(String::from("Sum:"));

                tree.begin_child(String::from("Left:"));

                left.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Right:"));

                right.build_tree(tree);

                tree.end_child();

                tree.end_child();
            },
            BinaryOperation::Product(left, right) => {
                tree.begin_child(String::from("Product:"));

                tree.begin_child(String::from("Left:"));

                left.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Right:"));

                right.build_tree(tree);

                tree.end_child();

                tree.end_child();
            },
            BinaryOperation::LessThan(left, right) => {
                tree.begin_child(String::from("LessThan:"));

                tree.begin_child(String::from("Left:"));

                left.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Right:"));

                right.build_tree(tree);

                tree.end_child();

                tree.end_child();
            },
            BinaryOperation::GreaterThan(left, right) => {
                tree.begin_child(String::from("GreaterThan:"));

                tree.begin_child(String::from("Left:"));

                left.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Right:"));

                right.build_tree(tree);

                tree.end_child();

                tree.end_child();
            },
            BinaryOperation::EqualityCheck(left, right) => {
                tree.begin_child(String::from("EqualityCheck:"));

                tree.begin_child(String::from("Left:"));

                left.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Right:"));

                right.build_tree(tree);

                tree.end_child();

                tree.end_child();
            },
        }
    }
}

impl Lit {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        match self {
            Lit::Int(str) => {
                tree.add_empty_child(format!("LitInt: {str}"));
            },
            Lit::Decimal(str) => {
                tree.add_empty_child(format!("LitDecimal: {str}"));
            },
            Lit::Char(str) => {
                tree.add_empty_child(format!("LitChar: {str}"));
            },
            Lit::String(str) => {
                tree.add_empty_child(format!("LitString: {str}"));
            },
        }
    }
}

impl Expression {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        match self {
            Expression::Lit(lit) => lit.build_tree(tree),
            Expression::Ident(id) => {
                tree.add_empty_child(format!("Ident: {id}"));
            }
            Expression::Deref(expr) => {
                tree.begin_child(String::from("Deref:"));

                expr.build_tree(tree);

                tree.end_child();
            }
            Expression::Reference(expr) => {
                tree.begin_child(String::from("Reference:"));

                expr.build_tree(tree);

                tree.end_child();
            }
            Expression::Index(base, index) => {
                tree.begin_child(String::from("Reference:"));

                tree.begin_child(String::from("Base:"));

                base.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Index:"));

                index.build_tree(tree);

                tree.end_child();
                tree.end_child();
            }
            Expression::FunctionCall(function_call) => {
                function_call.build_tree(tree);
            }
            Expression::BinaryOperation(binop) => binop.build_tree(tree),
            Expression::FieldAccess(expr, id) => {
                tree.begin_child(String::from("Field Access:"));

                tree.begin_child(String::from("Struct:"));

                expr.build_tree(tree);

                tree.end_child();

                tree.add_empty_child(format!("Field: {id}"));

                tree.end_child();
            }
        }
    }
}

impl Statement {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        match self {
            Statement::Declaration(ty, id) => {
                tree.begin_child(String::from("Declaration:"));

                tree.add_empty_child(format!("Type: {ty}"));
                tree.add_empty_child(format!("Ident: {id}"));

                tree.end_child();
            }
            Statement::Initialization(ty, id, expr) => {
                tree.begin_child(String::from("Initialization:"));

                tree.add_empty_child(format!("Type: {ty}"));
                tree.add_empty_child(format!("Ident: {id}"));
                tree.begin_child(String::from("Value:"));

                expr.build_tree(tree);

                tree.end_child();
                tree.end_child();
            }
            Statement::Assignment(id, expr) => {
                tree.begin_child(String::from("Assignment:"));

                tree.add_empty_child(format!("Ident: {id}"));
                tree.begin_child(String::from("Value:"));

                expr.build_tree(tree);

                tree.end_child();
                tree.end_child();
            }
            Statement::Expression(expr) => {
                tree.begin_child(String::from("Expression:"));

                expr.build_tree(tree);

                tree.end_child();
            }
            Statement::Return(expr) => match expr {
                Some(expr) => {
                    tree.begin_child(String::from("Return:"));

                    expr.build_tree(tree);

                    tree.end_child();
                }
                None => {
                    tree.add_empty_child(String::from("Return"));
                }
            },
            Statement::Break => {
                tree.add_empty_child(String::from("Break"));
            }
            Statement::Continue => {
                tree.add_empty_child(String::from("Continue"));
            }
            Statement::Braces(stmts) => {
                tree.begin_child(String::from("Braces:"));

                for stmt in stmts.iter() {
                    stmt.build_tree(tree);
                }

                tree.end_child();
            }
            Statement::IfStatement(condition, pos, neg) => {
                tree.begin_child(String::from("If Statement:"));

                tree.begin_child(String::from("Condition:"));

                condition.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("True:"));

                pos.build_tree(tree);

                tree.end_child();

                if let Some(neg) = neg {
                    tree.begin_child(String::from("False:"));

                    neg.build_tree(tree);

                    tree.end_child();
                }

                tree.end_child();
            }
            Statement::WhileStatement(condition, body) => {
                tree.begin_child(String::from("While Statement:"));

                tree.begin_child(String::from("Condition:"));

                condition.build_tree(tree);

                tree.end_child();

                tree.begin_child(String::from("Body:"));

                body.build_tree(tree);

                tree.end_child();

                tree.end_child();
            }
            Statement::ForStatement(init, condition, step, body) => {
                tree.begin_child(String::from("While Statement:"));

                tree.begin_child(String::from("Initialization:"));

                init.as_ref().inspect(|init| init.build_tree(tree));

                tree.end_child();

                tree.begin_child(String::from("Condition:"));

                condition.as_ref().inspect(|condition| condition.build_tree(tree));

                tree.begin_child(String::from("Step:"));

                step.as_ref().inspect(|step| step.build_tree(tree));

                tree.end_child();

                tree.end_child();

                tree.begin_child(String::from("Body:"));

                body.build_tree(tree);

                tree.end_child();

                tree.end_child();
            }
        }
    }
}

impl Function {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        tree.begin_child(String::from("Function:"));

        tree.add_empty_child(format!("Return Type: {}", self.return_type));
        tree.add_empty_child(format!("Ident: {}", self.ident));

        tree.begin_child(String::from("Params:"));

        for param in self.params.iter() {
            tree.add_empty_child(param.to_string());
        }

        if self.params.is_empty() {
            tree.add_empty_child(String::from("void"));
        }

        tree.end_child();

        tree.begin_child(String::from("Body:"));

        for stmt in self.body.iter() {
            stmt.build_tree(tree);
        }

        tree.end_child();
        tree.end_child();
    }
}

impl StructDefinition {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        tree.begin_child(String::from("Struct Definition:"));

        tree.add_empty_child(format!("Ident: {}", self.ident));

        tree.begin_child(String::from("Fields:"));

        for (id, ty) in self.ty.fields.iter() {
            tree.add_empty_child(format!("({ty}) {id}"));
        }

        tree.end_child();

        tree.end_child();
    }
}

impl Item {
    pub fn build_tree(&self, tree: &mut TreeBuilder) {
        match self {
            Item::Function(function) => function.build_tree(tree),
            Item::StructDefinition(ty) => ty.build_tree(tree),
        }
    }
}

impl Program {
    pub fn build_tree(&self) -> StringItem {
        let mut tree = TreeBuilder::new(String::from("Program:"));
        for item in self.root_items.iter() {
            item.build_tree(&mut tree);
        }
        tree.build()
    }
}
