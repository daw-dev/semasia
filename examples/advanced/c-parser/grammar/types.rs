use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::grammar::{
    language::NonArrayType,
    tokens::Ident,
};

use super::language::BaseType;

#[derive(Debug, Clone)]
pub struct StructType {
    pub fields: HashMap<Ident, Type>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Array(Box<Type>),
    Pointer(Box<Type>),
    Base(BaseType),
    Struct(StructType),
    // Function(Box<Type>, Vec<Type>),
}

impl From<NonArrayType> for Type {
    fn from(value: NonArrayType) -> Self {
        match value {
            NonArrayType::Pointer(inner) => Self::Pointer(Box::new((*inner).into())),
            NonArrayType::BaseType(base) => Self::Base(base),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedIdent {
    pub ident: Ident,
    pub ty: Type,
}

impl Display for TypedIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.ty, self.ident)
    }
}

impl Type {
    pub fn deref(self) -> Self {
        match self {
            Self::Pointer(ty) | Self::Array(ty) => *ty,
            _ => panic!("type cannot be deferenced"),
        }
    }

    pub fn compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Pointer(left), Type::Pointer(right))
            | (Type::Pointer(left), Type::Array(right))
            | (Type::Array(left), Type::Pointer(right))
            | (Type::Array(left), Type::Array(right)) => left.compatible_with(right),
            (Type::Base(left), Type::Base(right)) => left.compatible_with(right),
            _ => false,
        }
    }
}

impl BaseType {
    pub fn compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            (BaseType::Void, _) => true,
            (BaseType::Int, BaseType::Float) => true,
            (BaseType::Int, BaseType::Int) => true,
            (BaseType::Int, _) => false,
            (BaseType::Float, BaseType::Float) => true,
            (BaseType::Float, _) => false,
            (BaseType::Ident(left), BaseType::Ident(right)) => left == right,
            _ => false,
        }
    }
}

impl Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::Int => write!(f, "int"),
            BaseType::Float => write!(f, "float"),
            BaseType::Char => write!(f, "char"),
            BaseType::Void => write!(f, "void"),
            BaseType::Ident(ty) => write!(f, "{ty}"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Pointer(ty) => write!(f, "{ty}*"),
            Type::Array(ty) => write!(f, "{ty}[]"),
            Type::Base(base) => write!(f, "{base}"),
            Type::Struct(ty) => write!(
                f,
                "{{ {} }}",
                ty.fields
                    .iter()
                    .map(|(id, ty)| format!("({ty}) {id};"))
                    .format(" ")
            ),
        }
    }
}
