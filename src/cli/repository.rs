use super::Password;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct PasswordRepository {
    root_dir: PathBuf,
}

impl Default for PasswordRepository {
    fn default() -> Self {
        let root_dir = match std::env::var("PASSWORDS_PATH") {
            Ok(path) => PathBuf::from(&path),
            Err(_) => Password::default_path(),
        };
        PasswordRepository { root_dir }
    }
}

impl PasswordRepository {
    pub fn new() -> Self {
        PasswordRepository::default()
    }

    // TODO: implement password versioning

    pub fn add(&self, password: &Password) {
        if self.get(password.name()).is_some() {
            println!(
                "Password already exists. Delete before adding a new one."
            );

            return;
        }

        let mut file_path = self.root_dir.clone();
        file_path.push(password.name());

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .expect("Couldn't open file");

        writeln!(file, "{}", password.value())
            .expect("Couldn't save password");
    }

    pub fn get(&self, password_name: &str) -> Option<Password> {
        let mut file_path = self.root_dir.clone();
        file_path.push(password_name);

        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let mut reader = BufReader::new(file);
        let mut password_string = String::new();

        reader
            .read_line(&mut password_string)
            .expect("Error reading password");

        Some(Password::new(
            password_name.to_string(),
            password_string.trim().to_string(),
        ))
    }

    pub fn list(&self) {
        let paths = match fs::read_dir(&self.root_dir) {
            Ok(paths) => paths,
            Err(_) => {
                println!(
                    "Root path {} not found. Try running\npwm init\n.",
                    self.root_dir.display()
                );
                return;
            }
        };

        for path in paths {
            let path = path.expect("Failed to read file names");
            println!("{}", path.file_name().to_str().unwrap());
        }
    }

    pub fn remove(&self, password_name: &str) {
        let mut root_path = self.root_dir.clone();
        root_path.push(password_name);

        if root_path.exists() {
            fs::remove_file(root_path).expect("Couldn't remove password");
        }
    }

    pub fn update(&self, password: &Password) {
        self.remove(password.name());
        self.add(password);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_update_new_password() {
        let password =
            Password::new("test-password".to_string(), "TEST".to_string());
        let password_repo = PasswordRepository::new();

        password_repo.remove(&password.name());
        password_repo.add(&password);

        assert_eq!(
            password,
            password_repo
                .get(password.name())
                .expect("Couldn't get value")
        );

        let new_password =
            Password::new(password.name().to_string(), "NEW-TEST".to_string());
        password_repo.update(&new_password);

        assert_eq!(
            new_password,
            password_repo
                .get(&new_password.name())
                .expect("Couldn't get value")
        );

        password_repo.remove(new_password.name());
    }
}
