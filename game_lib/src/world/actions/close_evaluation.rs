#![allow(non_snake_case)]

use uuid::Uuid;

use crate::cards::types::card_model::Card;
use crate::cards::types::evaluation::EvaluationCard;
use crate::world::actions::action_error::ActionError::{InvalidState, WrongCardType};
use crate::world::actions::action_error::ActionResult;
use crate::world::board::Board;

pub fn close_evaluation_card(board: Board, card_id: &Uuid) -> ActionResult<Board> {
    if let Some(card) = board.open_cards.clone().get(card_id) {
        match &**card {
            Card::Evaluation(ec) => close_if_allowed(card_id, ec, board),
            Card::Attack(_) | Card::Oopsie(_) | Card::Lucky(_) | Card::Event(_) => Err(WrongCardType(board)),
        }
    } else {
        Err(InvalidState(board))
    }
}

fn close_if_allowed(card_id: &Uuid, ec: &EvaluationCard, board: Board) -> ActionResult<Board> {
    if ec.is_closeable() {
        let open_cards = &mut board.open_cards.clone();
        open_cards.remove(card_id);
        Ok(Board {
            open_cards: open_cards.clone(),
            ..board
        })
    } else {
        Err(InvalidState(board))
    }
}

#[cfg(test)]
mod tests {

    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::cards::properties::effect::Effect;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::card_model::Card;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::lucky::LuckyCard;
    use crate::cards::types::lucky::tests::FakeLuckyCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::evaluation::EvaluationCard;
    use crate::cards::types::evaluation::tests::FakeEvaluationCard;
    use crate::world::actions::action_error::ActionError;
    use crate::world::actions::close_evaluation::close_evaluation_card;
    use crate::world::board::Board;
    use crate::world::board::tests::{generate_board_with_open_card, remove_card_from_open_cards};

    #[test]
    fn try_close_non_open_card() {
        let (_, board, _) =
            generate_board_with_open_card(Card::from(FakeEvaluationCard.fake::<EvaluationCard>()));

        let expected_board = Board { ..board.clone() };

        let result = close_evaluation_card(board, &Uuid::new_v4()).unwrap_err();

        assert_eq!(result, ActionError::InvalidState(expected_board))
    }

    #[rstest]
    #[case::LuckyCard(Card::from(FakeLuckyCard.fake::< LuckyCard > ()))]
    #[case::AttackCard(Card::from(FakeAttackCard.fake::< AttackCard > ()))]
    #[case::OopsieCard(Card::from(FakeOopsieCard.fake::< OopsieCard > ()))]
    #[case::OopsieCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    fn try_close_wrong_card_type(#[case] card: Card) {
        let (card_id, board, _) = generate_board_with_open_card(card);

        let expected_board = Board { ..board.clone() };

        let result = close_evaluation_card(board, &card_id).unwrap_err();

        assert_eq!(result, ActionError::WrongCardType(expected_board));
    }


    #[test]
    fn close_noop_effect() {
        let evaluation_card = EvaluationCard {
            effect: Effect::NOP,
            ..FakeEvaluationCard.fake()
        };
        let card = Card::from(evaluation_card);

        let (card_id, board, _) = generate_board_with_open_card(card);

        let expected_board = Board {
            drawn_card: board.drawn_card.clone(),
            open_cards: remove_card_from_open_cards(&board, &card_id),
            ..Board::empty()
        };

        let result = close_evaluation_card(board, &card_id).unwrap();

        assert_eq!(result, expected_board);
    }
}
