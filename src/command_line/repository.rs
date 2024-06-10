use super::Password;
use std::fs::{File, OpenOptions};
use std::io;
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
        PasswordRepository { root_dir}
    }
}

impl PasswordRepository {
    pub fn new() -> Self {
        PasswordRepository::default()
    }

    pub fn add(&self, password: &Password) {
        if self.get(password.name()).is_some() {
            println!("Password already exists. Delete before adding a new one.")
        }

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .expect("Couldn't open file");

        writeln!(file, "{password}").expect("Couldn't save password");
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
        reader.read_line(&mut password_string).expect("Error reading password");

        None
    }

    pub fn list(&self) {
        let file = File::open(&self.path).expect("Couldn't open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Couldn't read lines");
            let mut splits = line.split(": ");
            let name = splits.nth(0).unwrap().to_string();
            println!("{name}")
        }
    }

    pub fn delete(&self, password_name: &str) {
        let temp_string =
            self.read_all_passwords_except(password_name).unwrap();
        self.write_all_passwords_from_string(temp_string)
            .expect("Couldn't write to file");
    }

    fn read_all_passwords_except(
        &self,
        password_name: &str,
    ) -> Option<String> {
        let file_to_read = match OpenOptions::new().read(true).open(&self.path)
        {
            Ok(file) => file,
            Err(err) => {
                println!("Couldn't open passwords file: {err}");
                return None;
            }
        };
        let reader = BufReader::new(&file_to_read);

        let temp_string =
            self.delete_from_lines(reader.lines(), password_name);

        Some(temp_string)
    }

    fn delete_from_lines<B: BufRead>(
        &self,
        lines: io::Lines<B>,
        element_to_delete: &str,
    ) -> String {
        let mut temp_string = String::new();
        for line in lines {
            let current_line = line.expect("Couldn't read lines");
            let split = current_line.split(": ").nth(0).unwrap().to_string();
            if split != element_to_delete {
                temp_string.push_str(&current_line);
                temp_string.push('\n');
            } else {
                println!("Deleted {element_to_delete}");
            }
        }

        temp_string
    }

    fn write_all_passwords_from_string(
        &self,
        temp_string: String,
    ) -> Result<(), io::Error> {
        let mut file_to_write = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
        {
            Ok(file) => file,
            Err(err) => {
                println!("Couldn't open passwords file: {err}");
                return Err(io::Error::from(err));
            }
        };
        file_to_write
            .write_all(temp_string.as_bytes())
            .expect("Couldn't write to file");

        Ok(())
    }

    pub fn update(&self, password: &Password) {
        // NOTE: could make own implementation for performance boost

        self.delete(password.name());
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

        password_repo.delete(&password.name());
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

        password_repo.delete(new_password.name());
    }
}
