// For development purposes
#![allow(unused_imports, dead_code, unused_variables)]

pub mod password;

use password::{Password, PasswordBuilder, PasswordRepository};
use std::env::args;

pub fn main() {
    let mut args = args();

    match args.nth(1) {
        Some(subcommand) => match subcommand.as_str() {
            "new" => {
                let password_name =
                    args.next().expect("No name for the password");
                match args.next() {
                    Some(password_value) => save_password(&Password::new(
                        password_name,
                        password_value,
                    )),
                    None => create_and_store_password(password_name),
                }
            }
            "get" => {
                get_password(&args.next().expect("No name for the password"))
            }
            "remove" => {
                remove_password(
                    &args.next().expect("No name for the password"),
                );
            }
            _ => panic!("Unknown subcommand {subcommand}"),
        },
        None => panic!("No subcommand found"),
    };
}

fn create_password(password_name: String) -> Password {
    let password_builder = PasswordBuilder::new();
    password_builder.build(password_name)
}

fn save_password(password: &Password) {
    let password_repository = PasswordRepository::new();
    password_repository.add(password);
}

fn create_and_store_password(password_name: String) {
    let password = create_password(password_name);
    save_password(&password);
    println!("{}", password)
}

fn get_password(password_name: &str) {
    let password_repository = PasswordRepository::new();
    let password = password_repository
        .get(password_name)
        .expect("Password not found!");
    println!("{}", password)
}

fn remove_password(password_name: &str) {
    let password_repository = PasswordRepository::new();
    password_repository.delete(password_name);
}
