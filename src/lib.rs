use std::env::args;

use password::PasswordBuilder;

pub mod password;

pub fn main() {
    let password_builder = PasswordBuilder;
    let password_name = args().nth(1).unwrap();
    let password = password_builder.build(password_name);
    println!("{}", password)
}