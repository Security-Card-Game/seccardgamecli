use uuid::Uuid;

use crate::world::actions::action_error::{ActionError, ActionResult};
use crate::world::board::{Board, CardRcWithId};
use crate::world::deck::{CardRc, Deck};

#[derive(Debug)]
struct CardAndNewDeck {
    drawn_card: CardRc,
    new_deck: Deck,
}

pub(crate) fn draw_card_and_place_on_board(
    deck: Deck,
    board: Board,
) -> ActionResult<(Deck, Board)> {
    let CardAndNewDeck {
        drawn_card,
        new_deck,
    } = draw_card(deck)?;
    let new_board = add_drawn_card_to_board(board, drawn_card)?;
    Ok((new_deck, new_board))
}

fn draw_card(deck: Deck) -> ActionResult<CardAndNewDeck> {
    return if deck.remaining_cards.is_empty() {
        Err(ActionError::NoCardsLeft)
    } else {
        let (drawn_card, remaining_cards) = deck.remaining_cards.split_at(1);
        let new_deck = Deck {
            remaining_cards: remaining_cards.to_vec(),
            played_cards: deck.played_cards + 1,
            total: deck.total,
        };
        Ok(CardAndNewDeck {
            drawn_card: drawn_card[0].clone(),
            new_deck,
        })
    };
}

fn add_drawn_card_to_board(board: Board, card: CardRc) -> ActionResult<Board> {
    let card_rc = CardRcWithId {
        id: Uuid::new_v4(),
        card: card.clone(),
    };
    let all_open_cards = &mut board.open_cards.clone();
    all_open_cards.insert(card_rc.id, card_rc.card.clone());

    Ok(Board {
        drawn_card: Some(card_rc),
        open_cards: all_open_cards.clone(),
        ..board
    })
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use fake::Fake;
    use crate::cards::types::card_model::Card;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::world::board::Board;
    use crate::world::deck::Deck;
    use crate::world::resources::Resources;

    use super::*;

    #[test]
    fn draw_card_from_deck() {
        let next_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let remaining_card: CardRc = Card::from(FakeEventCard.fake::<EventCard>()).into();
        let cards = vec![next_card.clone().into(), remaining_card.clone()];

        let deck = Deck::new(cards);

        let CardAndNewDeck {
            drawn_card: _,
            new_deck: deck_after_draw,
        } = draw_card(deck).unwrap();

        // assert deck has one played card
        assert_eq!(deck_after_draw.played_cards, 1);
        // assert deck has only on card left and it is the remaining card
        assert_eq!(deck_after_draw.remaining_cards.len(), 1);
        assert!(deck_after_draw.remaining_cards.contains(&remaining_card));
        assert_eq!(deck_after_draw.total, 2);
    }

    #[test]
    fn draw_card_from_deck_no_cards_left() {
        let next_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let cards = vec![next_card.clone()];

        let deck = Deck::new(cards.iter().map(|c| Rc::new(c.clone()) ).collect());

        let CardAndNewDeck {
            drawn_card: _,
            new_deck: deck_after_first_draw,
        } = draw_card(deck).unwrap();
        let result = draw_card(deck_after_first_draw);

        assert!(matches!(result, Err(ActionError::NoCardsLeft)));
    }

    #[test]
    fn add_card_to_open_cards() {
        let next_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let next_card_rc = Rc::new(next_card);

        let board = Board {
            current_resources: Resources::new(10),
            turns_remaining: 10,
            ..Board::empty()
        };

        let new_board = add_drawn_card_to_board(board, next_card_rc.clone()).unwrap();

        // unchanged values
        assert_eq!(new_board.current_resources, Resources::new(10));
        assert_eq!(new_board.turns_remaining, 10);

        // updated values
        assert_eq!(new_board.drawn_card.unwrap().card, next_card_rc);
        assert_eq!(new_board.open_cards.len(), 1);
        assert!(new_board
            .open_cards
            .iter()
            .find(|(_k, v)| **v == next_card_rc)
            .is_some());
    }

    #[test]
    fn action_draw_card_and_place_on_board() {
        let next_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let remaining_card = Card::from(FakeEventCard.fake::<EventCard>());
        let cards = vec![next_card.clone(), remaining_card.clone()];

        let deck = Deck::new(cards.iter().map(|c| Rc::new(c.clone()) ).collect());
        let board = Board::empty();

        let (_, board_after_draw) = draw_card_and_place_on_board(deck, board).unwrap();

        // assert board
        let next_card_rc = Rc::from(next_card.clone()); // board is only managing references
        assert_eq!(board_after_draw.open_cards.len(), 1);
        assert!(board_after_draw
            .open_cards
            .iter()
            .find(|(_k, v)| **v == next_card_rc)
            .is_some());
        assert_eq!(board_after_draw.drawn_card.unwrap().card, next_card_rc)
    }
}
