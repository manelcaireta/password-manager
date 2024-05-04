mod builder;
mod length;

pub struct PasswordBuilder;
pub struct Password {
    name: String,
    value: String,
}



impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}