pub mod builders;
pub mod savers;
mod length;

pub struct Password {
    name: String,
    value: String,
}

impl std::fmt::Display for Password {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.name, self.value)
    }
}

impl Password {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
