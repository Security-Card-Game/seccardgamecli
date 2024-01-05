use std::fmt;
use std::fmt::Formatter;
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    GameCloneError,
    CardCreationError,
    FileSystemError,
    ConfigError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::GameCloneError => write!(f, "GameCloneError"),
            ErrorKind::FileSystemError => write!(f, "FileSystemError"),
            ErrorKind::CardCreationError => write!(f, "CardCreationError"),
            ErrorKind::ConfigError => write!(f, "ConfigError"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CliError {
    pub kind: ErrorKind,
    pub message: String,
    pub original_message: Option<String>,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.original_message {
            Some(orig) => write!(f, "[{}]: {} (Reason: {})", self.kind, self.message, orig),
            None => write!(f, "[{}]: {}", self.kind, self.message),
        }
    }
}

impl CliError {
    pub fn new(kind: ErrorKind, message: &str, original_message: Option<String>) -> Self  {
        CliError {
            kind,
            message: message.to_string(),
            original_message
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;
