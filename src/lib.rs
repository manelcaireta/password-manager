// For development purposes
#![allow(unused_imports, dead_code, unused_variables)]

pub mod password;

use password::builders::PasswordBuilder;
use password::repository::PasswordRepository;
use password::Password;
use std::env::args;

pub fn main() {
    let mut args = args();

    match args.nth(1) {
        Some(subcommand) => match subcommand.as_str() {
            "new" => create_and_store_password(
                args.next().expect("No name for the password"),
            ),
            _ => panic!("Unknown subcommand {subcommand}"),
        },
        None => panic!("No subcommand found"),
    };
}

fn create_and_store_password(password_name: String) {
    let password = create_password(password_name);
    save_password(&password);
    println!("{}", password)
}

fn create_password(password_name: String) -> Password {
    let password_builder = PasswordBuilder::new();
    password_builder.build(password_name)
}

fn save_password(password: &Password) {
    let password_repository = PasswordRepository::new();
    password_repository.add(password);
}
