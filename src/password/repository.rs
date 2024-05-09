use super::Password;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

struct PasswordRepository {
    path: PathBuf,
}

impl Default for PasswordRepository {
    fn default() -> Self {
        let mut path = match std::env::var("PASSWORDS_PATH") {
            Ok(path) => PathBuf::from(&path),
            Err(_) => Password::default_path(),
        };
        path.push("pwords.txt");
        PasswordRepository { path }
    }
}

impl PasswordRepository {
    fn new() -> Self {
        PasswordRepository::default()
    }

    fn add(&self, password: &Password) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .expect("Couldn't open file");
        writeln!(file, "{password}").expect("Couldn't save password");
    }

    fn get(&self, password_name: &str) -> Result<Password, ()> {
        let file = File::open(&self.path).expect("Couldn't open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Couldn't read lines");
            let mut splits = line.split(": ");
            let name = splits.nth(0).unwrap().to_string();
            if name == password_name {
                let value = splits.next().unwrap().to_string();
                return Ok(Password::new(name, value));
            }
        }

        Err(())
    }

    fn delete(&self, password_name: &str) {
        let mut temp_string = String::new();
        let file_to_read = match OpenOptions::new().read(true).open(&self.path)
        {
            Ok(file) => file,
            Err(err) => {
                println!("Couldn't open passwords file: {err}");
                return;
            }
        };
        let reader = BufReader::new(&file_to_read);

        for line in reader.lines() {
            let current_line = line.expect("Couldn't read lines");
            let split = current_line.split(": ").nth(0).unwrap().to_string();
            if split != password_name {
                temp_string.push_str(&current_line);
                temp_string.push('\n');
            }
        }

        let mut file_to_write = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
        {
            Ok(file) => file,
            Err(err) => {
                println!("Couldn't open passwords file: {err}");
                return;
            }
        };
        file_to_write
            .write_all(temp_string.as_bytes())
            .expect("Couldn't write to file");
    }

    fn update(&self, password: &Password) {
        println!("updated");
    }
}

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
                .get(&password.name)
                .expect("Couldn't get value")
        );

        // let new_password =
        //     Password::new(password.name, "NEW-TEST".to_string());
        // password_repo.update(&new_password);

        // assert_eq!(
        //     new_password,
        //     password_repo
        //         .get(&new_password.name)
        //         .expect("Couldn't get value")
        // );

        password_repo.delete(&password.name);
    }
}
