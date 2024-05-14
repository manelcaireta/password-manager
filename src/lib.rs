pub mod command_line;

use command_line::CommandLine;
pub use command_line::{Password, PasswordBuilder, PasswordRepository};

pub fn main() {
    let mut command_line = CommandLine::new();
    command_line.run();
}
