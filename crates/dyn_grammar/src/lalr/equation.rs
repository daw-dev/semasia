#[derive(Clone)]
pub struct SymbolicSet {
    var_id: usize,
    var_dependencies: usize,
    tokens: Vec<String>,
}

#[derive(Default)]
pub struct EquationSet {
    equations: Vec<SymbolicSet>,
}

impl EquationSet {

}

