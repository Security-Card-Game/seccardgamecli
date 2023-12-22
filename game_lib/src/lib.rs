use std::io;

pub mod cards;
pub mod file;

pub fn print_to_stderr(message: &str) {
    let output = io::stderr();
    writeln!(output, "{}", message).unwrap();
}