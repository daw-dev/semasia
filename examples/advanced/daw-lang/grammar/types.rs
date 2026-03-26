use crate::grammar::tokens::Ident;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
    BaseType(Ident),
    Void,
    Function(Box<Type>, Vec<Type>),
}

impl Type {
    pub fn char() -> Self {
        Self::BaseType("char".into())
    }
    pub fn int() -> Self {
        Self::BaseType("int".into())
    }
    pub fn long() -> Self {
        Self::BaseType("long".into())
    }
    pub fn float() -> Self {
        Self::BaseType("float".into())
    }
    pub fn double() -> Self {
        Self::BaseType("double".into())
    }
    pub fn string() -> Self {
        Self::Pointer(Box::new(Self::char()))
    }

    pub fn deref(self) -> Self {
        match self {
            Self::Pointer(ty) | Self::Array(ty, _) => *ty,
            _ => panic!("type cannot be deferenced"),
        }
    }

    fn is_integer(ident: &Ident) -> bool {
        ident == "char" || ident == "int" || ident == "long"
    }

    fn is_decimal(ident: &Ident) -> bool {
        ident == "float" || ident == "double"
    }

    fn is_base_compatible(left: &Ident, right: &Ident) -> bool {
        Self::is_integer(left) && Self::is_integer(right)
            || Self::is_decimal(left) && (Self::is_decimal(right) || Self::is_integer(right))
    }

    pub fn compatible_with(&self, other: &Self) -> bool {
        self == other
            || match (self, other) {
                (Type::Pointer(left), Type::Pointer(right))
                | (Type::Pointer(left), Type::Array(right, _))
                | (Type::Array(left, _), Type::Pointer(right))
                | (Type::Array(left, _), Type::Array(right, _)) => left.compatible_with(right),
                (Type::BaseType(left), Type::BaseType(right)) => {
                    Self::is_base_compatible(left, right)
                }
                (Type::Void, _) => true,
                _ => false,
            }
    }
}
