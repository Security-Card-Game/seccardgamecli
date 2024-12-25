use uuid::Uuid;

use crate::cards::types::card_model::Card;
use crate::cards::types::lucky::LuckyCard;
use crate::world::actions::action_error::ActionError::{InvalidState, WrongCardType};
use crate::world::actions::action_error::ActionResult;
use crate::world::board::Board;

pub fn close_lucky_card(board: Board, card_id: &Uuid) -> ActionResult<Board> {
    if let Some(card) = board.open_cards.clone().get(card_id) {
        match &**card {
            Card::Lucky(lc) => close_if_allowed(card_id, lc, board),
            Card::Attack(_) | Card::Oopsie(_) | Card::Event(_) | Card::Evaluation(_) => Err(WrongCardType(board)),
        }
    } else {
        Err(InvalidState(board))
    }
}

fn close_if_allowed(card_id: &Uuid, lc: &LuckyCard, board: Board) -> ActionResult<Board> {
    if lc.is_closeable() {
        let open_cards = &mut board.open_cards.clone();
        open_cards.remove(card_id);
        let cards_to_use = &mut board.cards_to_use.clone();
        cards_to_use.remove(card_id);
        Ok(Board {
            open_cards: open_cards.clone(),
            cards_to_use: cards_to_use.clone(),
            ..board
        })
    } else {
        Err(InvalidState(board))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::cards::properties::effect::Effect;
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::cost_modifier::tests::FakeCostModifier;
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
    use crate::world::actions::close_lucky::close_lucky_card;
    use crate::world::board::Board;
    use crate::world::board::tests::generate_board_with_open_card;

    #[test]
    fn try_close_non_open_card() {
        let (_, board, _) =
            generate_board_with_open_card(Card::from(FakeLuckyCard.fake::<LuckyCard>()));

        let expected_board = Board { ..board.clone() };

        let result = close_lucky_card(board, &Uuid::new_v4()).unwrap_err();

        assert_eq!(result, ActionError::InvalidState(expected_board))
    }

    #[rstest]
    #[case::EventCard(Card::from(FakeEventCard.fake::< EventCard > ()))]
    #[case::AttackCard(Card::from(FakeAttackCard.fake::< AttackCard > ()))]
    #[case::OopsieCard(Card::from(FakeOopsieCard.fake::< OopsieCard > ()))]
    #[case::OopsieCard(Card::from(FakeEvaluationCard.fake::<EvaluationCard>()))]
    fn try_close_wrong_card_type(#[case] card: Card) {
        let (card_id, board, _) = generate_board_with_open_card(card);

        let expected_board = Board { ..board.clone() };

        let result = close_lucky_card(board, &card_id).unwrap_err();

        assert_eq!(result, ActionError::WrongCardType(expected_board));
    }

    #[test]
    fn try_effect_for_next_fix() {
        let effect = Effect::OnNextFix(FakeEffectDescription.fake(), FakeCostModifier.fake());
        let effect_card = LuckyCard {
            effect,
            ..FakeLuckyCard.fake()
        };
        let card = Card::from(effect_card);

        let (card_id, board, _) = generate_board_with_open_card(card);

        let expected_board = Board { ..board.clone() };

        let result = close_lucky_card(board, &card_id).unwrap_err();

        assert_eq!(result, ActionError::InvalidState(expected_board));
    }

    #[test]
    fn close_noop_effect() {
        let effect_card = LuckyCard {
            effect: Effect::NOP,
            ..FakeLuckyCard.fake()
        };
        let card = Card::from(effect_card);

        let (card_id, board, _) = generate_board_with_open_card(card);

        let expected_board = Board {
            open_cards: HashMap::new(),
            ..board.clone()
        };

        let result = close_lucky_card(board, &card_id).unwrap();

        assert_eq!(result, expected_board);
    }
}
