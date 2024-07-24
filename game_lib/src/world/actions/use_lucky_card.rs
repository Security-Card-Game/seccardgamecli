use uuid::Uuid;

use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::ActionError::{InvalidState, WrongCardType};
use crate::world::actions::action_error::ActionResult;
use crate::world::board::Board;

pub fn activate_lucky_card(board: Board, id: &Uuid) -> ActionResult<Board> {
    check_id_and_perform(board, id, add_lucky_card)
}

pub fn deactivate_lucky_card(board: Board, id: &Uuid) -> ActionResult<Board> {
    check_id_and_perform(board, id, remove_lucky_card)
}

fn check_id_and_perform<F>(board: Board, id: &Uuid, func: F) -> ActionResult<Board>
where
    F: FnOnce(Board, &Uuid) -> Board,
{
    if let Some(card) = board.open_cards.get(id) {
        match &**card {
            Card::Event(_) | Card::Attack(_) | Card::Oopsie(_) => Err(WrongCardType(board)),
            Card::Lucky(_) => Ok(func(board, id)),
        }
    } else {
        Err(InvalidState(board))
    }
}

fn add_lucky_card(board: Board, id: &Uuid) -> Board {
    let active_cards = &mut board.cards_to_use.clone();
    active_cards.insert(id.clone());
    Board {
        cards_to_use: active_cards.clone(),
        ..board
    }
}

fn remove_lucky_card(board: Board, id: &Uuid) -> Board {
    let active_cards = &mut board.cards_to_use.clone();
    active_cards.remove(id);
    Board {
        cards_to_use: active_cards.clone(),
        ..board
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::card_model::Card;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::lucky::tests::FakeLuckyCard;
    use crate::cards::types::lucky::LuckyCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::oopsie::OopsieCard;

    use crate::world::actions::action_error::ActionError;
    use crate::world::actions::use_lucky_card::{activate_lucky_card, deactivate_lucky_card};
    use crate::world::board::Board;
    use fake::Fake;
    use rstest::rstest;
    use std::rc::Rc;
    use uuid::Uuid;

    #[test]
    fn activate_existing_lucky_card() {
        let lucky_card = Card::from(FakeLuckyCard.fake::<LuckyCard>());
        let lucky_card_rc = Rc::new(lucky_card.clone());
        let lucky_card_id = Uuid::new_v4();

        let cards = vec![(lucky_card_id.clone(), lucky_card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            ..Board::empty()
        };
        let expected = Board {
            cards_to_use: vec![lucky_card_id].into_iter().collect(),
            ..board.clone()
        };

        let updated_board = activate_lucky_card(board, &lucky_card_id).unwrap();

        assert_eq!(updated_board, expected)
    }

    #[test]
    fn activate_existing_lucky_card_twice() {
        let lucky_card = Card::from(FakeLuckyCard.fake::<LuckyCard>());
        let lucky_card_rc = Rc::new(lucky_card.clone());
        let lucky_card_id = Uuid::new_v4();

        let cards = vec![(lucky_card_id.clone(), lucky_card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            ..Board::empty()
        };
        let expected = Board {
            cards_to_use: vec![lucky_card_id].into_iter().collect(),
            ..board.clone()
        };

        let updated_board = activate_lucky_card(board, &lucky_card_id).unwrap();
        let updated_board = activate_lucky_card(updated_board, &lucky_card_id).unwrap();

        assert_eq!(updated_board, expected)
    }

    #[test]
    fn activate_non_existing_card() {
        let lucky_card = Card::from(FakeLuckyCard.fake::<LuckyCard>());
        let lucky_card_rc = Rc::new(lucky_card.clone());

        let cards = vec![(Uuid::new_v4(), lucky_card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            ..Board::empty()
        };

        let result = activate_lucky_card(board.clone(), &Uuid::new_v4());

        if let Some(err) = result.err() {
            match err {
                ActionError::InvalidState(b) => {
                    assert_eq!(b, board)
                }
                _ => panic!("Expected invalid state!")
            }
        } else {
            println!("Expected and error!");
        }
    }

    #[rstest]
    #[case::AttackCard(Card::from(FakeAttackCard.fake::<AttackCard>()))]
    #[case::EventCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    #[case::OopsieCard(Card::from(FakeOopsieCard.fake::<OopsieCard>()))]
    fn activate_wrong_card_type(#[case] card: Card) {
        let card_rc = Rc::new(card.clone());
        let card_id = Uuid::new_v4();
        let cards = vec![(card_id.clone(), card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            ..Board::empty()
        };

        let result = activate_lucky_card(board.clone(), &card_id);

        if let Some(err) = result.err() {
            match err {
                ActionError::WrongCardType(b) => {
                    assert_eq!(b, board)
                }
                _ => panic!("Expected WrongCardType"),
            }
        } else {
            println!("Expected and error!");
        }
    }

    #[test]
    fn deactivate_existing_lucky_card() {
        let lucky_card = Card::from(FakeLuckyCard.fake::<LuckyCard>());
        let lucky_card_rc = Rc::new(lucky_card.clone());
        let lucky_card_id = Uuid::new_v4();

        let cards = vec![(lucky_card_id.clone(), lucky_card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            cards_to_use: vec![lucky_card_id].into_iter().collect(),
            ..Board::empty()
        };
        let expected = Board {
            cards_to_use: HashSet::new(),
            ..board.clone()
        };

        let updated_board = deactivate_lucky_card(board, &lucky_card_id).unwrap();

        assert_eq!(updated_board, expected)
    }

    #[test]
    fn deactivate_existing_lucky_card_twice() {
        let lucky_card = Card::from(FakeLuckyCard.fake::<LuckyCard>());
        let lucky_card_rc = Rc::new(lucky_card.clone());
        let lucky_card_id = Uuid::new_v4();

        let cards = vec![(lucky_card_id.clone(), lucky_card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            cards_to_use: vec![lucky_card_id].into_iter().collect(),
            ..Board::empty()
        };
        let expected = Board {
            cards_to_use: HashSet::new(),
            ..board.clone()
        };

        let updated_board = deactivate_lucky_card(board, &lucky_card_id).unwrap();
        let updated_board = deactivate_lucky_card(updated_board, &lucky_card_id).unwrap();

        assert_eq!(updated_board, expected)
    }

    #[test]
    fn deactivate_non_existing_card() {
        let lucky_card = Card::from(FakeLuckyCard.fake::<LuckyCard>());
        let lucky_card_rc = Rc::new(lucky_card.clone());

        let cards = vec![(Uuid::new_v4(), lucky_card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            ..Board::empty()
        };

        let result = deactivate_lucky_card(board.clone(), &Uuid::new_v4());

        if let Some(err) = result.err() {
            match err {
                ActionError::InvalidState(b) => {
                    assert_eq!(b, board)
                },
                _ => panic!("Expected InvalidState"),
            }
        } else {
            println!("Expected and error!");
        }
    }

    #[rstest]
    #[case::AttackCard(Card::from(FakeAttackCard.fake::<AttackCard>()))]
    #[case::EventCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    #[case::OopsieCard(Card::from(FakeOopsieCard.fake::<OopsieCard>()))]
    fn deactivate_wrong_card_type(#[case] card: Card) {
        let card_rc = Rc::new(card.clone());
        let card_id = Uuid::new_v4();
        let cards = vec![(card_id.clone(), card_rc)];

        let board = Board {
            open_cards: cards.into_iter().collect(),
            ..Board::empty()
        };

        let result = deactivate_lucky_card(board.clone(), &card_id);

        if let Some(err) = result.err() {
            match err {
                ActionError::WrongCardType(b) => {
                    assert_eq!(b, board)
                },
                _ => panic!("Expected WrongCardType")
            }
        } else {
            println!("Expected and error!");
        }
    }

}
