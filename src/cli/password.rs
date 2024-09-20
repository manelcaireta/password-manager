use std::path::PathBuf;

#[derive(PartialEq, Debug)]
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
        Password { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Password {
    pub fn default_path() -> PathBuf {
        match std::env::var("PASSWORD_HOME") {
            Ok(home_path) => PathBuf::from(&home_path),
            Err(_) => {
                let home_string = std::env::var("HOME").unwrap();
                let home_path =
                    PathBuf::from_iter([&home_string, ".password"]);
                home_path
            }
        }
    }

    pub fn create_home_directory() {
        let home_path = Password::default_path();
        if !home_path.exists() {
            std::fs::create_dir(&home_path).unwrap();
        }
    }
}
