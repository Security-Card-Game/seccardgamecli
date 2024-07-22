use std::error::Error;
use std::fmt::{Display, Formatter};

pub type ActionResult<T> = Result<T, ActionError>;

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    NoCardsLeft,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionError {
    NoCardsLeft,
}

impl Error for ActionError {}

impl Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ActionError::NoCardsLeft => "No cards lef in deck",
        };
        write!(f, "GameError: {:?}: {}", self, message)
    }
}
