use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::cards::types::card_model::Card;
use crate::world::board::{CardRcWithId, CurrentBoard};
use crate::world::deck::{CardRc, Deck};

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
        let card_rc = CardRcWithId {
            id: Uuid::new_v4(),
            card: Rc::new(card.clone()),
        };
        let all_open_cards = self.add_new_card_to_open_cards(&card_rc);
        Self {
            drawn_card: Some(card_rc),
            open_cards: all_open_cards.clone(),
            ..self
        }
    }

    fn add_new_card_to_open_cards(&self, card_rc: &CardRcWithId) -> HashMap<Uuid, CardRc> {
        let all_open_cards = &mut self.open_cards.clone();
        all_open_cards.insert(card_rc.id, card_rc.card.clone());
        all_open_cards.clone()
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use crate::cards::types::event::EventCard;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::world::board::CurrentBoard;
    use crate::world::deck::Deck;
    use crate::world::resources::Resources;

    use super::*;

    #[test]
    fn add_new_card_to_open_cards() {
        let next_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let remaining_card = Card::from(FakeEventCard.fake::<EventCard>());
        let cards = vec!(next_card.clone(), remaining_card.clone());

        let deck = Deck::new(cards);
        let board = CurrentBoard::init(deck.clone(), Resources::new(10));

        // guard: board has no open cards when initialized
        assert_eq!(board.open_cards.len(), 0);

        let (deck_after_draw, board_after_draw) = draw_card_and_place_on_board(deck, board);

        // assert deck has one played card
        assert_eq!(deck_after_draw.played_cards, 1);
        // assert deck has only on card left and it is the remaining card
        assert_eq!(deck_after_draw.remaining_cards.len(), 1);
        assert!(deck_after_draw.remaining_cards.contains(&remaining_card));
        assert_eq!(deck_after_draw.total, 2);

        // assert board
        let next_card_rc = Rc::from(next_card.clone()); // board is only managing references
        assert_eq!(board_after_draw.open_cards.len(), 1);
        assert!(board_after_draw.open_cards
            .iter()
            .find(|(k,v)| **v == next_card_rc)
            .is_some());
        assert_eq!(board_after_draw.drawn_card.unwrap().card, next_card_rc)
    }

}



