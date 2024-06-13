mod builders;
mod password;
mod repository;

pub use builders::PasswordBuilder;
pub use password::Password;
pub use repository::PasswordRepository;
use std::process::exit;

pub struct CommandLineInterface<I: Iterator<Item = String>> {
    args: I,
    builder: PasswordBuilder,
    repository: PasswordRepository,
}

impl<I: Iterator<Item = String>> CommandLineInterface<I> {
    pub fn from_iter(args: I) -> CommandLineInterface<I> {
        CommandLineInterface {
            args,
            builder: PasswordBuilder::new(),
            repository: PasswordRepository::new(),
        }
    }
}

impl CommandLineInterface<std::env::Args> {
    pub fn new() -> CommandLineInterface<std::env::Args> {
        let mut args = std::env::args();
        args.next();
        CommandLineInterface::from_iter(args)
    }
}

impl<I: Iterator<Item = String>> CommandLineInterface<I> {
    pub fn run(&mut self) {
        match self.args.next() {
            Some(subcommand) => match subcommand.as_str() {
                "get" => self.get_password(),
                "new" => self.new_password(),
                "remove" => self.remove_password(),
                "list" => self.list_all_passwords(),
                "gen" => self.generate_password(),
                "init" => Self::passwords_setup(),
                "help" => Self::show_documentation(),
                _ => {
                    eprintln!("pwm: Unknown subcommand {subcommand}\n");
                    Self::show_documentation();
                    exit(1);
                }
            },
            None => {
                eprintln!("pwm: No subcommand provided\n");
                Self::show_documentation();
                exit(1);
            }
        }
    }

    fn get_password(&mut self) {
        let password_name = self.password_name_from_args();
        let password = match self.repository.get(&password_name) {
            Some(password) => password,
            None => {
                eprintln!("pwm: Password {password_name} not found");
                exit(1);
            }
        };
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
        let password_name = match self.args.next() {
            Some(password_name) => password_name,
            None => {
                eprintln!("pwm: No password name provided");
                exit(1);
            }
        };

        password_name
    }

    fn passwords_setup() {
        Password::create_home_directory();
    }

    fn show_documentation() {
        let width = 10;

        println!("usage: pwm <command>");
        println!("");
        println!("Commands:");
        println!("  {:width$} Initializes password manager", "init");
        println!("  {:width$} Generates a password on the fly without", "gen");
        println!("  {:width$} storing its value", "");
        println!("  {:width$} Creates and stores a new password", "new");
        println!("  {:width$} Lists all passwords", "list");
        println!("  {:width$} Recovers the value of a password", "get");
        println!("  {:width$} Removes a password", "remove");
        println!("  {:width$} Shows this help", "help");
    }
}
