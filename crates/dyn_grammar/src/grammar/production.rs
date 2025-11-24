use std::fmt::Display;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Production {
    name: String,
    head: String,
    body: Vec<String>,
}

impl Production {
    pub fn new(name: String, head: String, body:Vec<String>) -> Self {
        Self {
            name,
            head,
            body,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn arity(&self) -> usize {
        self.body.len()
    }
}

impl Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} -> ({})", self.name, self.head, self.body.iter().format(", "))
    }
}
