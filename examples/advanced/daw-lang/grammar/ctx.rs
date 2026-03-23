use std::collections::HashMap;

use crate::grammar::{tokens::Ident, types::Type};

pub type SymbolTable = HashMap<Ident, Type>;

#[derive(Default, Debug)]
pub struct CompilationContext {
    symbol_table: SymbolTable,
}

impl CompilationContext {
    pub fn declare(&mut self, ident: &Ident, ty: &Type) {
        if self
            .symbol_table
            .insert(ident.clone(), ty.clone())
            .is_some()
        {
            panic!("ident is already declared!");
        }
    }

    pub fn get_type(&self, ident: &Ident) -> Option<Type> {
        self.symbol_table.get(ident).cloned()
    }
}
