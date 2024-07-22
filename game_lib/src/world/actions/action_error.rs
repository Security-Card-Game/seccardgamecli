use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::world::board::Board;

pub type ActionResult<T> = Result<T, ActionError>;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionError {
    NoCardsLeft,
    WrongCardType(Board),
    AttackForceClosed(Board),
    InvalidState(Board),
}

impl Error for ActionError {}

impl Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ActionError::NoCardsLeft => "No cards left in deck",
            ActionError::WrongCardType(_) => "Wrong card type",
            ActionError::AttackForceClosed(_) => "Attack forced to be over!",
            ActionError::InvalidState(_) => "This would lead to an invalid state!"
        };
        write!(f, "GameError: {:?}: {}", self, message)
    }
}
