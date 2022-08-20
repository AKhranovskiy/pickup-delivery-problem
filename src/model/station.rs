#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Station {
    name: String,
}

impl Station {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&str> for Station {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
