use super::password::Password;
use super::version::PasswordVersion;
use std;
use std::fs::{self, create_dir, read_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::exit;

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
        if self.get(password.name()).is_ok() {
            /*
             * TODO:
             * get latest version
             * update new version
             */

            return;
        }

        let mut file_path = self.root_dir.clone();
        file_path.push(password.name());

        /* Literally 1984 */
        create_dir(file_path);

        file_path.push("1");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .expect("Couldn't open file");

        writeln!(file, "{}", password.value())
            .expect("Couldn't save password");
    }

    pub fn get(
        &self,
        password_name: &str,
    ) -> Result<PasswordVersion, Box<dyn std::error::Error>> {
        self.get_latest_version(password_name)
    }

    pub fn get_latest_version(
        &self,
        password_name: &str,
    ) -> Result<PasswordVersion, Box<dyn std::error::Error>> {
        let mut password_versions_path =
            self.root_dir.join(Path::new(password_name));

        let mut current_version: u32;

        for entry in read_dir(password_versions_path)? {
            let entry = entry?;
            let version = match entry.file_name().into_string() {
                Ok(version) => version.parse::<u32>()?,
                Err(_) => {
                    eprintln!("pwm: Corrupted file detected in password {password_name} folder");
                    continue;
                }
            };

            if current_version < version {
                current_version = version;
            }
        }

        let password_path = password_versions_path
            .join(Path::new(current_version.to_string().as_str()));

        let password_value = fs::read_to_string(password_path)?;
        let password =
            Password::new(password_name.to_string(), password_value);
        Ok(PasswordVersion::new(password, current_version))
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
        let password_version = PasswordVersion::new(password, 1);
        let password_repo = PasswordRepository::new();

        password_repo.remove(&password.name());
        password_repo.add(&password);

        assert_eq!(
            password_version,
            password_repo
                .get(password.name())
                .expect("Couldn't get value")
        );

        let new_password =
            Password::new(password.name().to_string(), "NEW-TEST".to_string());
        password_repo.update(&new_password);
        let new_password_version = PasswordVersion::new(new_password, 1);

        assert_eq!(
            new_password_version,
            password_repo
                .get(&new_password.name())
                .expect("Couldn't get value")
        );

        password_repo.remove(new_password.name());
    }
}
