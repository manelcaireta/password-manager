use super::Password;

#[derive(PartialEq, Debug)]
pub struct PasswordVersion {
    password: Password,
    version: u32,
}

impl PasswordVersion {
    pub fn new(password: Password, version: u32) -> Self {
        PasswordVersion { password, version }
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn register_new_version(&mut self) {
        self.version += 1
    }
}

impl std::fmt::Display for PasswordVersion {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(formatter, "{}", self.password)
    }
}