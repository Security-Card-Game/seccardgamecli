use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::cards::properties::fix_cost::FixCost;
use crate::world::board::Board;
use crate::world::resources::Resources;

pub type ActionResult<T> = Result<T, ActionError>;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionError {
    NoCardsLeft,
    WrongCardType(Board),
    AttackForceClosed(Board),
    InvalidState(Board),
    NotEnoughResources(Board, Resources)
}

impl Error for ActionError {}

impl Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ActionError::NoCardsLeft => "No cards left in deck".to_string(),
            ActionError::WrongCardType(_) => "Wrong card type".to_string(),
            ActionError::AttackForceClosed(_) => "Attack forced to be over!".to_string(),
            ActionError::InvalidState(_) => "This would lead to an invalid state!".to_string(),
            ActionError::NotEnoughResources(_, costs) => format!("Not enough resources, fix would have cost {}", costs)
        };
        write!(f, "GameError: {:?}: {}", self, message)
    }
}
