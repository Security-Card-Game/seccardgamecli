use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::cards::properties::cost_modifier::CostModifier;
use crate::world::deck::{CardRc, Deck};
use crate::world::reputation::Reputation;
use crate::world::resources::Resources;
/*
The Board is the current state of the game. It holds all the data present on the board. And also if
a card will be used by the players. It does not contain information which actions are possible. It merly
is a container for the current state.

All properties of the board should be not mutable. After every round or action a new board is generated.
 */
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CardRcWithId {
    pub id: Uuid,
    pub card: CardRc,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub current_resources: Resources,
    pub current_reputation: Reputation,
    pub(crate) drawn_card: Option<CardRcWithId>,
    pub open_cards: HashMap<Uuid, CardRc>,
    pub cards_to_use: HashSet<Uuid>,
    pub cost_modifier: Option<CostModifier>,
    pub turns_remaining: usize,
}

impl Board {
    pub fn init(deck: &Deck, start_resources: Resources, start_reputation: Reputation) -> Self {
        Board {
            current_resources: start_resources,
            current_reputation: start_reputation,
            drawn_card: None,
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            cost_modifier: None,
            turns_remaining: deck.total,
        }
    }

    pub fn empty() -> Self {
        Board {
            current_resources: Resources::new(0),
            current_reputation: Reputation::start_value(),
            drawn_card: None,
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            cost_modifier: None,
            turns_remaining: 0,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::rc::Rc;

    use uuid::Uuid;

    use crate::cards::types::card_model::Card;
    use crate::world::board::Board;

    pub fn generate_board_with_open_card(card: Card) -> (Uuid, Board, Rc<Card>) {
        let card_rc = Rc::new(card.clone());
        let card_id = Uuid::new_v4();

        let open_cards = vec![(card_id.clone(), card_rc.clone())];

        let board = Board {
            open_cards: open_cards.into_iter().collect(),
            ..Board::empty()
        };
        (card_id, board, card_rc)
    }
}
