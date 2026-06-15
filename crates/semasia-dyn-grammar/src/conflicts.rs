#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ProductionPriority {
    #[default]
    None,
    Inherited(usize),
    Explicit(usize),
}

#[test]
fn precedence_ordering_test() {
    assert!(ProductionPriority::Explicit(10) > ProductionPriority::Explicit(5));
    assert!(ProductionPriority::Inherited(10) > ProductionPriority::Inherited(5));
    assert!(ProductionPriority::Explicit(10) > ProductionPriority::Inherited(5));
    assert!(ProductionPriority::Explicit(5) > ProductionPriority::Inherited(10));
    assert!(ProductionPriority::Explicit(10) > ProductionPriority::None);
    assert!(ProductionPriority::Inherited(10) > ProductionPriority::None);
}

pub type TokenPriority = Option<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Associativity {
    #[default]
    Unspecified,
    Left,
    Right,
}
