use super::Password;

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

    pub fn register_version(&mut self, version: u32) {
        self.version += 1
    }
}