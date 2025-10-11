use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::cards::properties::duration::Duration;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::ActionError::WrongCardType;
use crate::world::actions::action_error::{ActionError, ActionResult};
use crate::world::board::Board;
use crate::world::deck::CardRc;

/*
Decreased the duration of all AttackCards with a limited duration. Removes the cards from the board
if they hit zero duration.
*/
pub fn update_attack_cards(board: Board) -> Board {
    let mut open_cards = HashMap::new();
    let drawn_card_id = board.drawn_card.clone().map(|card| card.id);
    for (key, card) in board.open_cards.iter() {
        let card_to_insert = match &**card {
            Card::Attack(ac) => handle_attack_card(drawn_card_id, key, card, ac),
            Card::Event(_) | Card::Oopsie(_) | Card::Lucky(_) | Card::Evaluation(_) => {
                Some(card.clone())
            }
        };

        match card_to_insert {
            None => {}
            Some(c) => {
                open_cards.insert(*key, c);
            }
        }
    }

    Board {
        open_cards: open_cards.clone(),
        ..board
    }
}

fn handle_attack_card(drawn_card_id: Option<Uuid>, key: &Uuid, card: &CardRc, ac: &AttackCard) -> Option<Rc<Card>> {
    match drawn_card_id {
        None => Some(card.clone()), // if no card was drawn nothing should change
        Some(id) => if key == &id {
            Some(card.clone())
        } else {
            decrease_duration(card, ac)
        }
    }
}

fn decrease_duration(card: &CardRc, ac: &AttackCard) -> Option<Rc<Card>> {
    let new_duration = ac.duration.decrease();
    if let Some(value) = new_duration.value() {
        if *value == 0 {
            return None;
        }
    };

    match new_duration {
        Duration::Rounds(_) => {
            let attack_card = AttackCard {
                title: ac.title.clone(),
                description: ac.description.clone(),
                effect: ac.effect.clone(),
                duration: new_duration,
            };
            Some(Rc::new(Card::from(attack_card)))
        }
        Duration::UntilClosed => Some(card.clone()),
        Duration::None => None,
    }
}

/// Try to close an attack card manually by its ID.
/// - If the card attack card has not zero duration, returns 'AttackForceClosed'.
/// - If the card is not found, return an `InvalidState` error.
/// - If it's the wrong type of card, return a `WrongCardType` error.
/// - Otherwise, close the attack card.
pub fn manually_close_attack_card(board: Board, card_id: &Uuid) -> ActionResult<Board> {
    if let Some(card) = board.open_cards.get(card_id) {
        match &**card {
            Card::Oopsie(_) | Card::Lucky(_) | Card::Event(_) | Card::Evaluation(_) => {
                Err(WrongCardType(board.clone()))
            }
            Card::Attack(ac) => close_attack_card(&ac.duration.clone(), board, card_id),
        }
    } else {
        Err(ActionError::InvalidState(board.clone()))
    }
}

fn close_attack_card(duration: &Duration, board: Board, id: &Uuid) -> ActionResult<Board> {
    let open_cards = &mut board.open_cards.clone();
    open_cards.remove(id);

    let updated_board = Board {
        open_cards: open_cards.clone(),
        ..board
    };

    match duration {
        Duration::Rounds(r) => {
            if r == &0usize {
                Ok(updated_board)
            } else {
                Err(ActionError::AttackForceClosed(updated_board))
            }
        }
        Duration::UntilClosed => Err(ActionError::AttackForceClosed(updated_board)),
        Duration::None => Ok(updated_board),
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::cards::properties::duration::Duration;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::card_model::Card;
    use crate::cards::types::evaluation::tests::FakeEvaluationCard;
    use crate::cards::types::evaluation::EvaluationCard;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::lucky::tests::FakeLuckyCard;
    use crate::cards::types::lucky::LuckyCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::world::actions::action_error::ActionError;
    use crate::world::actions::close_attack::{manually_close_attack_card, update_attack_cards};
    use crate::world::board::tests::{generate_board_with_freshly_drawn_card, generate_board_with_open_card, remove_card_from_open_cards};
    use crate::world::board::Board;

    #[test]
    fn update_attack_cards_reduces_attack_duration() {
        let attack = AttackCard {
            duration: Duration::new(Some(5)),
            ..FakeAttackCard.fake()
        };
        let expected_attack = AttackCard {
            duration: Duration::new(Some(4)),
            ..attack.clone()
        };

        let (card_id, board, card_rc) = generate_board_with_open_card(Card::from(attack));

        let board_after_update = update_attack_cards(board);

        let updated_card = board_after_update.open_cards.get(&card_id).unwrap();

        assert_eq!(&**updated_card, &Card::from(expected_attack));
        assert!(!Rc::ptr_eq(&card_rc, updated_card));
    }

    #[test]
    fn update_attack_cards_does_not_reduce_attack_duration_when_card_is_freshly_drawn() {
        let attack = AttackCard {
            duration: Duration::new(Some(5)),
            ..FakeAttackCard.fake()
        };
        let expected_attack = AttackCard {
            duration: Duration::new(Some(5)),
            ..attack.clone()
        };

        let (card_id, board, card_rc) = generate_board_with_freshly_drawn_card(Card::from(attack));

        let board_after_update = update_attack_cards(board);

        let updated_card = board_after_update.open_cards.get(&card_id).unwrap();

        assert_eq!(&**updated_card, &Card::from(expected_attack));
        assert!(Rc::ptr_eq(&card_rc, updated_card)); // card reference is not changed
    }


    #[test]
    fn update_attack_cards_removes_attack_when_duration_is_0() {
        let attack = AttackCard {
            duration: Duration::new(Some(1)),
            ..FakeAttackCard.fake()
        };
        let (id, board, _) = generate_board_with_open_card(Card::from(attack));

        let board_after_update = update_attack_cards(board);

        assert!(!board_after_update.open_cards.contains_key(&id));
    }

    #[test]
    fn update_attack_cards_removes_attack_when_duration_is_none() {
        let attack = AttackCard {
            duration: Duration::None,
            ..FakeAttackCard.fake()
        };

        let (id, board, _card_rc) = generate_board_with_open_card(Card::from(attack));

        let board_after_update = update_attack_cards(board);

        assert!(!board_after_update.open_cards.contains_key(&id));
    }

    #[test]
    fn update_attack_cards_does_not_affect_duration_until_closed() {
        let attack = AttackCard {
            duration: Duration::UntilClosed,
            ..FakeAttackCard.fake()
        };

        let (card_id, board, card_rc) = generate_board_with_open_card(Card::from(attack));

        let board_after_update = update_attack_cards(board);

        let card_after_update = board_after_update.open_cards.get(&card_id).unwrap();

        assert!(Rc::ptr_eq(&card_rc, card_after_update))
    }

    #[rstest]
    #[case::LuckyCard(Card::from(FakeLuckyCard.fake::<LuckyCard>()))]
    #[case::EventCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    #[case::OopsieCard(Card::from(FakeOopsieCard.fake::<OopsieCard>()))]
    #[case::OopsieCard(Card::from(FakeEvaluationCard.fake::<EvaluationCard>()))]
    fn update_attack_cards_does_not_affect_other_cards(#[case] card: Card) {
        let (_, board, _) = generate_board_with_open_card(card);

        let expected_board = Board { ..board.clone() };

        let result = update_attack_cards(board);

        assert_eq!(result, expected_board);
    }

    #[test]
    fn manually_close_attack_card_closes_card_and_returns_error_for_until_closed() {
        let attack = AttackCard {
            duration: Duration::UntilClosed,
            ..FakeAttackCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(attack));

        let expected_board = Board {
            drawn_card: board.drawn_card.clone(),
            open_cards: remove_card_from_open_cards(&board, &card_id),
            ..Board::empty()
        };

        let err_result = manually_close_attack_card(board, &card_id).unwrap_err();

        assert_eq!(err_result, ActionError::AttackForceClosed(expected_board))
    }

    #[test]
    fn manually_close_attack_card_closes_card_and_returns_error_for_duration_not_0() {
        let attack = AttackCard {
            duration: Duration::Rounds(1),
            ..FakeAttackCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(attack));

        let expected_board = Board {
            drawn_card: board.drawn_card.clone(),
            open_cards: remove_card_from_open_cards(&board, &card_id),
            ..Board::empty()
        };

        let err_result = manually_close_attack_card(board, &card_id).unwrap_err();

        assert_eq!(err_result, ActionError::AttackForceClosed(expected_board))
    }

    #[test]
    fn manually_close_attack_card_closes_card_for_duration_0() {
        let attack = AttackCard {
            duration: Duration::Rounds(0),
            ..FakeAttackCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(attack));

        let expected_board = Board {
            drawn_card: board.drawn_card.clone(),
            open_cards: remove_card_from_open_cards(&board, &card_id),
            ..Board::empty()
        };

        let result = manually_close_attack_card(board, &card_id).unwrap();

        assert_eq!(result, expected_board)
    }

    #[test]
    fn manually_close_attack_card_closes_card_for_duration_none() {
        let attack = AttackCard {
            duration: Duration::None,
            ..FakeAttackCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(attack));

        let expected_board = Board {
            drawn_card: board.drawn_card.clone(),
            open_cards: remove_card_from_open_cards(&board, &card_id),
            ..Board::empty()
        };

        let result = manually_close_attack_card(board, &card_id).unwrap();

        assert_eq!(result, expected_board)
    }

    #[rstest]
    #[case::LuckyCard(Card::from(FakeLuckyCard.fake::<LuckyCard>()))]
    #[case::EventCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    #[case::OopsieCard(Card::from(FakeOopsieCard.fake::<OopsieCard>()))]
    #[case::OopsieCard(Card::from(FakeEvaluationCard.fake::<EvaluationCard>()))]
    fn manually_close_attack_card_closes_card_returns_error_for_wrong_type(#[case] card: Card) {
        let (card_id, board, _card_rc) = generate_board_with_open_card(card);

        let expected_board = Board { ..board.clone() };

        let result = manually_close_attack_card(board, &card_id).unwrap_err();

        assert_eq!(result, ActionError::WrongCardType(expected_board));
    }

    #[test]
    fn manually_close_attack_card_closes_card_returns_error_when_card_id_not_in_open_cards() {
        let attack = AttackCard {
            duration: Duration::None,
            ..FakeAttackCard.fake()
        };

        let (_card_id, board, _) = generate_board_with_open_card(Card::from(attack));
        let expected_board = Board { ..board.clone() };

        let result = manually_close_attack_card(board, &Uuid::new_v4()).unwrap_err();

        assert_eq!(result, ActionError::InvalidState(expected_board));
    }
}
