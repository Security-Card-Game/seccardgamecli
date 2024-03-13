use std::error::Error;
use std::fmt::{Display, Formatter};

pub type GameLibResult<T> = Result<T, GameLibError>;

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    IO,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameLibError {
    kind: ErrorKind,
    message: String,
    original_message: Option<String>,
}

impl Display for GameLibError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.original_message {
            None => write!(f, "GameLibError: {:?}: {}", self.kind, self.message),
            Some(orig) => write!(
                f,
                "GameLibError: {:?}: {} (was {})",
                self.kind, self.message, orig
            ),
        }
    }
}

impl Error for GameLibError {}

impl GameLibError {
    pub fn create(kind: ErrorKind, message: &str) -> Self {
        GameLibError {
            kind,
            message: message.to_string(),
            original_message: None,
        }
    }

    pub fn create_with_original(kind: ErrorKind, message: &str, original_message: String) -> Self {
        GameLibError {
            kind,
            message: message.to_string(),
            original_message: Some(original_message),
        }
    }
}
