use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::cards::properties::fix_modifier::FixModifier;
use crate::world::deck::{CardRc, Deck};
use crate::world::resources::Resources;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CardRcWithId {
    pub id: Uuid,
    pub card: CardRc,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub current_resources: Resources,
    pub(crate) drawn_card: Option<CardRcWithId>,
    pub open_cards: HashMap<Uuid, CardRc>,
    pub cards_to_use: HashSet<Uuid>,
    pub fix_modifier: Option<FixModifier>,
    pub turns_remaining: usize,
}

impl Board {
    pub fn init(deck: &Deck, start_resources: Resources) -> Self {
        Board {
            current_resources: start_resources,
            drawn_card: None,
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            fix_modifier: None,
            turns_remaining: deck.total,
        }
    }

    pub fn empty() -> Self {
        Board {
            current_resources: Resources::new(0),
            drawn_card: None,
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            fix_modifier: None,
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
