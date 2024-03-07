#[derive(Debug, Clone)]
pub struct WorldError {
    pub message: String,
}

impl WorldError {
    pub fn new() -> WorldError {
        WorldError {
            message: "Error".to_string(),
        }
    }

    pub fn with_message(message: &str) -> WorldError {
        WorldError {
            message: message.to_string(),
        }
    }
}

pub type WoldResult<T> = Result<T, WorldError>;
