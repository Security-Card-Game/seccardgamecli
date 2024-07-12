use crate::cards::properties::duration::Duration;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::world::board::{CardRcWithId, CurrentBoard};
use crate::world::deck::{CardRc, Deck};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub(crate) fn draw_card_and_place_on_board(
    deck: Deck,
    board: CurrentBoard,
) -> (Deck, CurrentBoard) {
    let (next_card, remaining_cards) = deck.remaining_cards.split_at(1);
    let new_board = board.add_drawn_card(&next_card[0]);
    let new_deck = Deck {
        remaining_cards: remaining_cards.to_vec(),
        played_cards: deck.played_cards + 1,
        total: deck.total,
    };
    (new_deck, new_board)
}

impl CurrentBoard {
    pub fn add_drawn_card(self, card: &Card) -> Self {
        let cardRc = CardRcWithId {
            id: Uuid::new_v4(),
            card: Rc::new(card.clone()),
        };
        let all_open_cards = self.add_new_card_to_open_cards(&cardRc);
        Self {
            drawn_card: Some(cardRc),
            open_cards: all_open_cards.clone(),
            ..self
        }
    }

    fn add_new_card_to_open_cards(&self, cardRc: &CardRcWithId) -> HashMap<Uuid, CardRc> {
        let all_open_cards = &mut self.open_cards.clone();
        all_open_cards.insert(cardRc.id, cardRc.card.clone());
        all_open_cards.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::deck::Deck;
    use crate::world::board::CurrentBoard;

    #[test]
    fn add_new_card_to_open_cards() {

    }

}



