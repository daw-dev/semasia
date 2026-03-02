#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Precedence {
    Implicit(usize),
    Explicit(usize),
}

#[test]
fn precedence_ordering_test() {
    assert!(Precedence::Explicit(10) > Precedence::Explicit(5));
    assert!(Precedence::Implicit(10) > Precedence::Implicit(5));
    assert!(Precedence::Explicit(10) > Precedence::Implicit(5));
    assert!(Precedence::Explicit(5) > Precedence::Implicit(10));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Associativity {
    Unspecified,
    Left,
    Right,
}
