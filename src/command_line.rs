mod builders;
mod password;
mod repository;

pub use builders::PasswordBuilder;
pub use password::Password;
pub use repository::PasswordRepository;

pub struct CommandLine<I: Iterator<Item = String>> {
    args: I,
    builder: PasswordBuilder,
    repository: PasswordRepository,
}

impl<I: Iterator<Item = String>> CommandLine<I> {
    pub fn from_iter(args: I) -> CommandLine<I> {
        CommandLine {
            args,
            builder: PasswordBuilder::new(),
            repository: PasswordRepository::new(),
        }
    }
}

impl CommandLine<std::env::Args> {
    pub fn new() -> CommandLine<std::env::Args> {
        let mut args = std::env::args();
        args.next();
        CommandLine::from_iter(args)
    }
}

impl<I: Iterator<Item = String>> CommandLine<I> {
    pub fn run(&mut self) {
        match self.args.next() {
            Some(subcommand) => match subcommand.as_str() {
                "get" => self.get_password(),
                "new" => self.new_password(),
                "remove" => self.remove_password(),
                "list" => self.list_all_passwords(),
                "gen" => self.generate_password(),
                "init" => Self::passwords_setup(),
                _ => {
                    println!("Unknown subcommand {subcommand}\n");
                    self.show_documentation();
                }
            },
            None => {
                // This will be changed to show documentation
                println!("No subcommand provided\n");
                self.show_documentation();
            }
        }
    }

    fn get_password(&mut self) {
        let password_name = self.password_name_from_args();
        let password = self
            .repository
            .get(&password_name)
            .expect("Password not found!");
        println!("{}", password)
    }

    fn new_password(&mut self) {
        let password_name = self.password_name_from_args();
        match self.args.next() {
            Some(password_value) => self
                .save_password(&Password::new(password_name, password_value)),
            None => self.create_and_save_password(password_name),
        }
    }

    fn create_and_save_password(&self, password_name: String) {
        let password = self.create_password(password_name);
        self.save_password(&password);
        println!("{}", password)
    }

    fn create_password(&self, password_name: String) -> Password {
        self.builder.build(password_name)
    }

    fn save_password(&self, password: &Password) {
        self.repository.add(password);
    }

    fn remove_password(&mut self) {
        let password_name = self.password_name_from_args();
        self.repository.delete(&password_name);
    }

    fn list_all_passwords(&self) {
        self.repository.list();
    }

    fn generate_password(&self) {
        let password = self.builder.build_secure_password();
        println!("{}", password);
    }

    fn password_name_from_args(&mut self) -> String {
        self.args.next().expect("No password name provided")
    }

    fn passwords_setup() {
        Password::create_home_directory();
    }

    fn show_documentation(&self) {
        let width = 10;

        println!("usage: pwm <command>");
        println!("");
        println!("Commands:");
        println!("  {:width$} Initializes password manager", "init");
        println!("  {:width$} Generates a password", "gen");
        println!("  {:width$} Creates and stores a new password", "new");
        println!("  {:width$} Lists all passwords", "list");
        println!("  {:width$} Recovers the value of a password", "get");
        println!("  {:width$} Removes a password", "remove")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_password() {
        let mut command_line = CommandLine::new();
        command_line.new_password();
    }
}
