mod builders;
mod flags;
mod password;
mod repository;
mod version;

pub use builders::PasswordBuilder;
use flags::GetFlags;
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
                "update" => self.update_password(),
                "remove" | "rm" => self.remove_password(),
                "list" => self.list_all_passwords(),
                "gen" => self.generate_password(),
                "init" => Self::passwords_setup(),
                "help" => Self::show_documentation(),
                _ => {
                    eprintln!("pwm: Unknown subcommand '{subcommand}'\n");
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
        let mut flags = GetFlags::new();

        let password = match self.repository.get(&password_name, flags) {
            Ok(password) => password,
            Err(_) => {
                eprintln!("pwm: Password {password_name} not found");
                exit(1);
            }
        };
        println!("{}", password)
    }

    pub fn parse_get_flags(
        &self,
        mut flags: GetFlags,
    ) -> GetFlags {
        match self.args.next() {
            None => return flags,
            Some(arg) => match arg.as_str() {
                "--version" => {
                    flags.version =
                        self.args.next().map(|s| match s.parse::<u32>() {
                            Ok(result) => result,
                            Err(_) => {
                                eprintln!(
                                    "pwm: Incorrect version number '{s}'"
                                );
                                std::process::exit(1)
                            }
                        })
                }
                value => {
                    eprintln!("pwm: Unknown flag '{value}' for get command\n")
                }
            },
        }

        self.parse_get_flags(flags)
    }

    fn new_password(&mut self) {
        let password_name = self.password_name_from_args();
        match self.args.next() {
            Some(password_value) => self
                .repository
                .add(&Password::new(password_name, password_value)),
            None => self.create_and_save_password(password_name),
        }
    }

    fn update_password(&mut self) {
        let password_name = self.password_name_from_args();
        match self.args.next() {
            Some(password_value) => self
                .repository
                .update(&Password::new(password_name, password_value)),
            None => self.create_and_update_password(password_name),
        };
    }

    fn create_and_save_password(&self, password_name: String) {
        let password = self.builder.build(password_name);
        self.repository.add(&password);
        println!("{}", password)
    }

    fn create_and_update_password(&self, password_name: String) {
        let password = self.builder.build(password_name);
        self.repository.update(&password);
        println!("{}", password)
    }

    fn remove_password(&mut self) {
        let password_name = self.password_name_from_args();

        println!(
            "Are you sure you want to delete the password? (yes/no) [no]",
        );
        let mut user_confirmation = String::new();
        match std::io::stdin().read_line(&mut user_confirmation) {
            Ok(_) => (),
            Err(_) => user_confirmation = String::from("no"),
        };

        if user_confirmation.trim().to_lowercase() == "yes" {
            self.repository.remove(&password_name);
        } else {
            println!("Password deletion aborted");
        }
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
        let width = 12;

        println!("usage: pwm <command>\n");
        println!("Commands:");
        println!("  {:width$} Initializes password manager", "init");
        println!("  {:width$} Generates a password on the fly without", "gen");
        println!("  {:width$} storing its value", "");
        println!("  {:width$} Creates and stores a new password", "new");
        println!("  {:width$} Lists all passwords", "list");
        println!("  {:width$} Recovers the value of a password", "get");
        println!("  {:width$} Removes a password", "rm, remove");
        println!("  {:width$} Shows this help", "help");
    }
}
