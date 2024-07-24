/*
TODO: Add public method to close an oopsie by randomly selecting a value between min and max of it, then
add the fix_modifier and then multiply it by reource_fix_modifier. Then return a new Board with
decreased resources and all used cards and cards with Effect:OnNextFix closed.

 If resources are not enough, return ActionError::NotEnoughRersources(Board)-
 */
use std::ops::Sub;
use rand::{Rng, thread_rng};
use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::ActionError::WrongCardType;
use crate::world::actions::action_error::{ActionError, ActionResult};
use crate::world::board::Board;
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use uuid::Uuid;
use crate::cards::properties::fix_cost::FixCost;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::oopsie::OopsieCard;
use crate::world::resources::Resources;

pub fn try_and_pay_for_oopsie_fix(
    board: Board,
    card_id: &Uuid,
    resource_fix_multiplier: ResourceFixMultiplier,
) -> ActionResult<Board> {
    if let Some(card) = board.open_cards.get(card_id) {
        match &**card {
            Card::Attack(_) | Card::Lucky(_) | Card::Event(_) => Err(WrongCardType(board.clone())),
            Card::Oopsie(oc) => try_and_close(&board, card_id, oc, resource_fix_multiplier),
        }
    } else {
        Err(ActionError::InvalidState(board.clone()))
    }
}

fn try_and_close(board: &Board, card_id: &Uuid, oopsie_card: &OopsieCard, resource_fix_multiplier: ResourceFixMultiplier) -> ActionResult<Board> {
    let base_fix_cost = roll_dice(&oopsie_card.fix_cost);
    let modified_fix_cost = apply_fix_modifier(&board, &base_fix_cost);
    let real_fix_costs = modified_fix_cost * &resource_fix_multiplier;
    if &board.current_resources >= &real_fix_costs {
        let new_open_cards = &mut board.open_cards.clone();
        new_open_cards.remove(card_id);
        Ok(Board {
            current_resources: &board.current_resources - &real_fix_costs,
            open_cards: new_open_cards.clone(),
            fix_modifier: None,
            ..board.clone()
        })
    } else {
        Err(ActionError::NotEnoughResources(Board {
            fix_modifier: None,
            current_resources: Resources::new(0),
            ..board.clone()
        }, real_fix_costs))
    }
}

fn apply_fix_modifier(board: &&Board, base_fix_cost: &Resources) -> Resources {
    if let Some(modifier) = &board.fix_modifier {
        match modifier {
            FixModifier::Increase(r) => base_fix_cost + r,
            FixModifier::Decrease(r) => base_fix_cost - r,
        }
    } else {
        base_fix_cost.clone()
    }
}

fn roll_dice(fix_cost: &FixCost) -> Resources {
    let mut rng = thread_rng();
    if (fix_cost.min == fix_cost.max) {
        fix_cost.min.clone()
    } else {
        let cost = rng.gen_range(fix_cost.min.value().clone()..fix_cost.max.value().clone());
        Resources::new(cost.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::cards::properties::fix_cost::FixCost;
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
    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;
    use crate::cards::properties::fix_modifier::FixModifier;
    use crate::world::actions::close_oopsie::try_and_pay_for_oopsie_fix;
    use crate::world::board::Board;
    use crate::world::board::tests::generate_board_with_open_card;
    use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
    use crate::world::resources::Resources;

    #[test]
    fn close_oopsie_card_returns_invalid_state_if_card_id_not_open() {
        let oopsie_card: OopsieCard = FakeOopsieCard.fake();
        let (_, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let expected_board = board.clone();

        let result = try_and_pay_for_oopsie_fix(board, &Uuid::new_v4(), ResourceFixMultiplier::default())
            .unwrap_err();

        assert_eq!(result, ActionError::InvalidState(expected_board));
    }

    #[rstest]
    #[case::LuckyCard(Card::from(FakeAttackCard.fake::<AttackCard>()))]
    #[case::EventCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    #[case::OopsieCard(Card::from(FakeLuckyCard.fake::<LuckyCard>()))]
    fn close_oopsie_card_returns_wrong_card_type_if_card_id_is_not_for_oopsie(#[case] card: Card) {
        let (card_id, board, _) = generate_board_with_open_card(card);
        let expected_board = board.clone();

        let result =
            try_and_pay_for_oopsie_fix(board, &card_id, ResourceFixMultiplier::default()).unwrap_err();

        assert_eq!(result, ActionError::WrongCardType(expected_board));
    }

    #[test]
    fn close_oopsie_card_returns_board_with_reduced_resources_and_closes_oopsie() {

        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(5, 5).unwrap(),
            ..FakeOopsieCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let board_with_resourecs = Board {
            current_resources: Resources::new(10),
            ..board
        };

        let expected_board = Board {
            current_resources: Resources::new(5),
            open_cards: HashMap::new(),
            fix_modifier: None,
            ..board_with_resourecs.clone()
        };

        let result = try_and_pay_for_oopsie_fix(board_with_resourecs, &card_id, ResourceFixMultiplier::default()).unwrap();

        assert_eq!(result, expected_board);
        assert!(result.fix_modifier.is_none());
    }

    #[test]
    fn close_oopsie_card_with_resource_multiplier_returns_board_with_reduced_resources_and_closes_oopsie() {

        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(5, 5).unwrap(),
            ..FakeOopsieCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let board_with_resourecs = Board {
            current_resources: Resources::new(10),
            ..board
        };

        let expected_board = Board {
            current_resources: Resources::new(0),
            open_cards: HashMap::new(),
            fix_modifier: None,
            ..board_with_resourecs.clone()
        };

        let result = try_and_pay_for_oopsie_fix(board_with_resourecs, &card_id, ResourceFixMultiplier::new(2)).unwrap();

        assert_eq!(result, expected_board);
    }

    #[test]
    fn close_oopsie_card_with_resource_multiplier_and_fix_modifier_randomly_costs_returns_board_with_reduced_resources_and_closes_oopsie() {

        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(5, 10).unwrap(),
            ..FakeOopsieCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let board_with_resourecs = Board {
            current_resources: Resources::new(24),
            fix_modifier: Some(FixModifier::Increase(Resources::new(2))),
            ..board
        };

        let result = try_and_pay_for_oopsie_fix(board_with_resourecs, &card_id, ResourceFixMultiplier::new(2)).unwrap();

        assert!(result.open_cards.is_empty());
        assert!(result.current_resources.value().clone() <= 10);
        assert!(result.fix_modifier.is_none());
    }

    #[test]
    fn close_oopsie_card_with_returns_error_if_not_enough_resources_and_sets_board_resources_to_0_keeps_oopsie_unchanged() {

        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(10, 10).unwrap(),
            ..FakeOopsieCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let board_with_resourecs = Board {
            current_resources: Resources::new(23),
            fix_modifier: Some(FixModifier::Increase(Resources::new(2))),
            ..board
        };

        let real_fix_costs = Resources::new(24);

        let expected_board = Board {
            current_resources: Resources::new(0),
            fix_modifier: None,
            ..board_with_resourecs.clone()
        };


        let result = try_and_pay_for_oopsie_fix(board_with_resourecs, &card_id, ResourceFixMultiplier::new(2)).unwrap_err();

        assert_eq!(result, ActionError::NotEnoughResources(expected_board, real_fix_costs))

    }



}
