#[derive(Clone)]
pub enum TokenAction<Prod> {
    Shift(usize),
    Reduce(Prod),
}

#[derive(Clone)]
pub enum EofAction<Prod> {
    Reduce(Prod),
    Accept,
}
