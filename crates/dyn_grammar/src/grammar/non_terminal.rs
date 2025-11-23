use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonTerminal {
    name: String,
}

impl NonTerminal {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Display for NonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
