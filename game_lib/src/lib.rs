use log::error;

pub mod cards;
pub mod file;

pub fn print_to_stderr(message: &str) {
    error!("{}", message);
}
