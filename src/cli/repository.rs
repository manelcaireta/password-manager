use super::flags::GetFlags;
use super::password::Password;
use super::version::PasswordVersion;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{self, fs, io};

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
        let password_folder = self.root_dir.join(Path::new(password.name()));

        if self.get_latest_version(&password_folder).is_ok() {
            eprintln!("pwm: Password already exists. To update en existing password run:\n\n  `pwm update <PASSWORD_NAME> [PASSWORD VALUE]`");
            std::process::exit(1);
        };

        fs::create_dir_all(&password_folder).expect("Expect");

        self.write_password_version(
            &password_folder,
            PasswordVersion::new(password.to_owned(), 1),
        );
    }

    pub fn get(
        &self,
        password_name: &str,
        options: GetFlags,
    ) -> Result<PasswordVersion, Box<dyn std::error::Error>> {
        let password_folder = self.root_dir.join(Path::new(password_name));
        let version = match options.version {
            None => self.get_latest_version(&password_folder)?,
            Some(version) => version,
        };

        let password_path =
            password_folder.join(Path::new(version.to_string().as_str()));

        let password_value = fs::read_to_string(password_path)?;
        let password =
            Password::new(password_name.to_string(), password_value);
        Ok(PasswordVersion::new(password, version))
    }

    fn get_latest_version<P: AsRef<Path>>(
        &self,
        password_folder: P,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let mut current_version: u32 = 0;

        for entry in fs::read_dir(&password_folder)? {
            let entry = entry?;
            let version = match entry.file_name().into_string() {
                Ok(version) => version.parse::<u32>()?,
                Err(_) => continue,
            };

            if current_version < version {
                current_version = version;
            }
        }

        if current_version == 0 {
            Err(Box::new(io::Error::from(io::ErrorKind::NotFound)))
        } else {
            Ok(current_version)
        }
    }

    pub fn update(&self, password: &Password) {
        let password_folder = self.root_dir.join(Path::new(password.name()));
        let version = match self.get_latest_version(&password_folder) {
            Ok(version) => version + 1,
            Err(_) => {
                eprintln!("pwm: Password does not exist. To create a new password run:\n\n  `pwm new <PASSWORD_NAME> [PASSWORD_VALUE]`");
                std::process::exit(1);
            }
        };

        self.write_password_version(
            &password_folder,
            PasswordVersion::new(password.to_owned(), version),
        );
    }

    fn write_password_version<P: AsRef<Path>>(
        &self,
        password_folder: P,
        password_version: PasswordVersion,
    ) {
        let password_file = password_folder
            .as_ref()
            .join(Path::new(password_version.version().to_string().as_str()));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&password_file)
            .expect("Couldn't open file");

        write!(file, "{}", password_version.password().value())
            .expect("Couldn't save password");
    }

    pub fn list(&self) {
        let paths = match fs::read_dir(&self.root_dir) {
            Ok(paths) => paths,
            Err(_) => {
                eprintln!(
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

        if root_path.exists() && root_path.is_dir() {
            fs::remove_dir_all(&root_path).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSWORD_NAME: &str = "test-password";
    const PASSWORD_VALUE: &str = "TEST";
    const NEW_PASSWORD_VALUE: &str = "NEW-TEST";

    #[test]
    fn save_and_update_new_password() {
        let password_repo = PasswordRepository::new();

        let password = Password::new(
            PASSWORD_NAME.to_string(),
            PASSWORD_VALUE.to_string(),
        );
        let password_version = PasswordVersion::new(password.clone(), 1);
        let mut original_options = GetFlags::new();
        original_options.version = Some(1);

        password_repo.remove(&password.name());

        let new_password = Password::new(
            PASSWORD_NAME.to_string(),
            NEW_PASSWORD_VALUE.to_string(),
        );
        let new_password_version =
            PasswordVersion::new(new_password.clone(), 2);
        let mut new_options = GetFlags::new();
        new_options.version = Some(2);

        password_repo.add(&password);
        password_repo.update(&new_password);

        assert_eq!(
            password_version,
            password_repo
                .get(&new_password.name(), original_options)
                .expect("Couldn't get value")
        );
        assert_eq!(
            new_password_version,
            password_repo
                .get(&new_password.name(), new_options)
                .expect("Couldn't get value")
        );

        password_repo.remove(new_password.name());
    }
}
