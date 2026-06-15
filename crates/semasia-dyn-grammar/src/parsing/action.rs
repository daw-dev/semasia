#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenAction {
    Shift(usize),
    Reduce(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EofAction {
    Reduce(usize),
    Accept,
}
