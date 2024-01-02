use std::io;
use std::io::Write;

pub mod cards;
pub mod file;

pub fn print_to_stderr(message: &str) {
    let mut output = io::stderr();
    writeln!(output, "{}", message).unwrap();
}
