pub mod cli;

use cli::CommandLineInterface;
pub use cli::{Password, PasswordBuilder, PasswordRepository};

pub fn main() {
    let mut command_line = CommandLineInterface::new();
    command_line.run();
}
