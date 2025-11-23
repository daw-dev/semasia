use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    name: String,
    regexpr: String,
}

impl Token {
    pub fn new(name: String, regexpr: String) -> Self {
        Self {
            name,
            regexpr,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}/ => {}", self.regexpr, self.name)
    }
}
