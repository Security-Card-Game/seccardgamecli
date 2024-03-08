use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct WorldError {
    pub message: String,
}

impl WorldError {
    pub fn create() -> WorldError {
        WorldError {
            message: "Error".to_string(),
        }
    }

    pub fn create_with_message(message: &str) -> WorldError {
        WorldError {
            message: message.to_string(),
        }
    }
}

impl Display for WorldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for WorldError {}

pub type WoldResult<T> = Result<T, WorldError>;
