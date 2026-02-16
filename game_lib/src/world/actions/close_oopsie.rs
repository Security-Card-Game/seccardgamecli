/*
TODO: Add public method to close an oopsie by randomly selecting a value between min and max of it, then
add the fix_modifier and then multiply it by reource_fix_modifier. Then return a new Board with
decreased resources and all used cards and cards with Effect:OnNextFix closed.

 If resources are not enough, return ActionError::NotEnoughRersources(Board)-
 */
use std::collections::HashSet;

use rand::{Rng, thread_rng};
use uuid::Uuid;

use crate::cards::properties::effect::Effect;
use crate::cards::properties::fix_cost::FixCost;
use crate::cards::properties::cost_modifier::CostModifier;
use crate::cards::types::card_model::{Card, CardTrait};
use crate::cards::types::oopsie::OopsieCard;
use crate::world::actions::action_error::{ActionError, ActionResult};
use crate::world::actions::action_error::ActionError::WrongCardType;
use crate::world::board::Board;
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

pub fn try_and_pay_for_oopsie_fix(
    board: Board,
    card_id: &Uuid,
    resource_fix_multiplier: ResourceFixMultiplier,
) -> ActionResult<(Board, Resources)> {
    if let Some(card) = board.open_cards.get(card_id) {
        match &**card {
            Card::Attack(_) | Card::Lucky(_) | Card::Event(_) | Card::Evaluation(_) => Err(WrongCardType(board.clone())),
            Card::Oopsie(oc) => try_and_close(&board, card_id, oc, resource_fix_multiplier),
        }
    } else {
        Err(ActionError::InvalidState(board.clone()))
    }
}

fn try_and_close(
    board: &Board,
    card_id: &Uuid,
    oopsie_card: &OopsieCard,
    resource_fix_multiplier: ResourceFixMultiplier,
) -> ActionResult<(Board, Resources)> {
    let base_fix_cost = roll_dice(&oopsie_card.fix_cost);
    let modified_fix_cost = apply_fix_modifier(&board, &base_fix_cost);
    let real_fix_costs = modified_fix_cost * &resource_fix_multiplier;

    let new_open_cards = &mut board.open_cards.clone();
    // remove used cards from board
    new_open_cards.retain(|id, card| {
        !board.cards_to_use.contains(id) && !matches!(card.effect(), Effect::OnNextFix(_, _))
    });

    if board.current_resources >= real_fix_costs {
        new_open_cards.remove(card_id);
        Ok((
            Board {
                current_resources: &board.current_resources - &real_fix_costs,
                open_cards: new_open_cards.clone(),
                cards_to_use: HashSet::new(),
                cost_modifier: None,
                ..board.clone()
            },
            real_fix_costs.clone(),
        ))
    } else {
        Err(ActionError::NotEnoughResources(
            Board {
                current_resources: Resources::new(0),
                open_cards: new_open_cards.clone(),
                cards_to_use: HashSet::new(),
                cost_modifier: None,
                ..board.clone()
            },
            real_fix_costs,
        ))
    }
}

fn apply_fix_modifier(board: &&Board, base_fix_cost: &Resources) -> Resources {
    if let Some(modifier) = &board.cost_modifier {
        match modifier {
            CostModifier::Increase(r) => base_fix_cost + r,
            CostModifier::Decrease(r) => base_fix_cost - r,
        }
    } else {
        base_fix_cost.clone()
    }
}

fn roll_dice(fix_cost: &FixCost) -> Resources {
    let mut rng = thread_rng();
    if fix_cost.min == fix_cost.max {
        fix_cost.min.clone()
    } else {
        let cost = rng.gen_range(*fix_cost.min.value()..*fix_cost.max.value());
        Resources::new(cost)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::cards::properties::effect::Effect;
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::fix_cost::FixCost;
    use crate::cards::properties::cost_modifier::CostModifier;
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
    use crate::world::actions::calculate_board::calculate_board;
    use crate::world::actions::close_oopsie::try_and_pay_for_oopsie_fix;
    use crate::world::board::Board;
    use crate::world::board::tests::{generate_board_with_open_card, remove_card_from_open_cards};
    use crate::world::deck::Deck;
    use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
    use crate::world::resources::Resources;

    #[test]
    fn close_oopsie_card_returns_invalid_state_if_card_id_not_open() {
        let oopsie_card: OopsieCard = FakeOopsieCard.fake();
        let (_, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let expected_board = board.clone();

        let result =
            try_and_pay_for_oopsie_fix(board, &Uuid::new_v4(), ResourceFixMultiplier::default())
                .unwrap_err();

        assert_eq!(result, ActionError::InvalidState(expected_board));
    }

    #[rstest]
    #[case::LuckyCard(Card::from(FakeAttackCard.fake::<AttackCard>()))]
    #[case::EventCard(Card::from(FakeEventCard.fake::<EventCard>()))]
    #[case::OopsieCard(Card::from(FakeLuckyCard.fake::<LuckyCard>()))]
    #[case::OopsieCard(Card::from(FakeEvaluationCard.fake::<EvaluationCard>()))]
    fn close_oopsie_card_returns_wrong_card_type_if_card_id_is_not_for_oopsie(#[case] card: Card) {
        let (card_id, board, _) = generate_board_with_open_card(card);
        let expected_board = board.clone();

        let result = try_and_pay_for_oopsie_fix(board, &card_id, ResourceFixMultiplier::default())
            .unwrap_err();

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
            open_cards: remove_card_from_open_cards(&board_with_resourecs, &card_id),
            cost_modifier: None,
            ..board_with_resourecs.clone()
        };

        let result = try_and_pay_for_oopsie_fix(
            board_with_resourecs,
            &card_id,
            ResourceFixMultiplier::default(),
        )
        .unwrap();

        assert_eq!(result, (expected_board, Resources::new(5)));
        assert!(result.0.cost_modifier.is_none());
    }

    #[test]
    fn close_oopsie_card_with_resource_multiplier_returns_board_with_reduced_resources_and_closes_oopsie(
    ) {
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
            open_cards: remove_card_from_open_cards(&board_with_resourecs, &card_id),
            cost_modifier: None,
            ..board_with_resourecs.clone()
        };

        let result = try_and_pay_for_oopsie_fix(
            board_with_resourecs,
            &card_id,
            ResourceFixMultiplier::new(2),
        )
        .unwrap();

        assert_eq!(result, (expected_board, Resources::new(10)));
    }

    #[test]
    fn close_oopsie_card_with_resource_multiplier_and_fix_modifier_randomly_costs_returns_board_with_reduced_resources_and_closes_oopsie(
    ) {
        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(5, 10).unwrap(),
            ..FakeOopsieCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let board_with_resourecs = Board {
            current_resources: Resources::new(24),
            cost_modifier: Some(CostModifier::Increase(Resources::new(2))),
            ..board
        };

        let result = try_and_pay_for_oopsie_fix(
            board_with_resourecs,
            &card_id,
            ResourceFixMultiplier::new(2),
        )
        .unwrap();

        assert!(!result.0.open_cards.contains_key(&card_id));
        assert!(result.0.current_resources.value().clone() <= 10);
        assert!(result.0.cost_modifier.is_none());
    }

    #[test]
    fn close_oopsie_card_removes_used_cards_and_oopsie_from_board_on_success() {
        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(10, 10).unwrap(),
            ..FakeOopsieCard.fake()
        };
        let oopsie_id = Uuid::new_v4();
        let oopsie_rc = Rc::new(Card::from(oopsie_card));

        let event_card = EventCard {
            effect: Effect::OnNextFix(
                FakeEffectDescription.fake(),
                CostModifier::Increase(Resources::new(5)),
            ),
            ..FakeEventCard.fake()
        };
        let event_id = Uuid::new_v4();
        let event_rc = Rc::new(Card::from(event_card));

        let lucky_card = LuckyCard {
            effect: Effect::OnUsingForFix(
                FakeEffectDescription.fake(),
                CostModifier::Decrease(Resources::new(4)),
            ),
            ..FakeLuckyCard.fake()
        };
        let lucky_id = Uuid::new_v4();
        let lucky_rc = Rc::new(Card::from(lucky_card));

        let open_cards = vec![
            (oopsie_id, oopsie_rc),
            (event_id, event_rc),
            (lucky_id, lucky_rc),
        ];

        let prepared_board = calculate_board(
            Board {
                open_cards: open_cards.into_iter().collect(),
                cards_to_use: vec![lucky_id].into_iter().collect(),
                current_resources: Resources::new(11),
                ..Board::empty()
            },
            &Deck {
                remaining_cards: vec![],
                played_cards: 10,
                total: 10,
            },
        );

        let expected_board = Board {
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            cost_modifier: None,
            current_resources: Resources::new(0),
            ..prepared_board.clone()
        };

        let result =
            try_and_pay_for_oopsie_fix(prepared_board, &oopsie_id, ResourceFixMultiplier::new(1))
                .unwrap();

        assert_eq!(result.0, expected_board);
        assert_eq!(result.1, Resources::new(11));
    }

    #[test]
    fn close_oopsie_card_removes_used_cards_but_not_oopsie_from_board_on_failure() {
        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(10, 10).unwrap(),
            ..FakeOopsieCard.fake()
        };
        let oopsie_id = Uuid::new_v4();
        let oopsie_rc = Rc::new(Card::from(oopsie_card));

        let event_card = EventCard {
            effect: Effect::OnNextFix(
                FakeEffectDescription.fake(),
                CostModifier::Increase(Resources::new(5)),
            ),
            ..FakeEventCard.fake()
        };
        let event_id = Uuid::new_v4();
        let event_rc = Rc::new(Card::from(event_card));

        let lucky_card = LuckyCard {
            effect: Effect::OnUsingForFix(
                FakeEffectDescription.fake(),
                CostModifier::Decrease(Resources::new(4)),
            ),
            ..FakeLuckyCard.fake()
        };
        let lucky_id = Uuid::new_v4();
        let lucky_rc = Rc::new(Card::from(lucky_card));

        let open_cards = vec![
            (oopsie_id, oopsie_rc.clone()),
            (event_id, event_rc),
            (lucky_id, lucky_rc),
        ];

        let prepared_board = calculate_board(
            Board {
                open_cards: open_cards.into_iter().collect(),
                cards_to_use: vec![lucky_id].into_iter().collect(),
                current_resources: Resources::new(10),
                ..Board::empty()
            },
            &Deck {
                remaining_cards: vec![],
                played_cards: 10,
                total: 10,
            },
        );

        dbg!("open cards: {}", prepared_board.clone().open_cards);

        let open_cards_after = vec!{
            (oopsie_id, oopsie_rc.clone())
        };

        let expected_board = Board {
            open_cards: open_cards_after.into_iter().collect(),
            cards_to_use: HashSet::new(),
            cost_modifier: None,
            current_resources: Resources::new(0),
            ..prepared_board.clone()
        };

        let result =
            try_and_pay_for_oopsie_fix(prepared_board, &oopsie_id, ResourceFixMultiplier::new(1))
                .unwrap_err();

        assert_eq!(result,  ActionError::NotEnoughResources(expected_board, Resources::new(11)));
    }


    #[test]
    fn close_oopsie_card_with_returns_error_if_not_enough_resources_and_sets_board_resources_to_0_keeps_oopsie_unchanged(
    ) {
        let oopsie_card: OopsieCard = OopsieCard {
            fix_cost: FixCost::new(10, 10).unwrap(),
            ..FakeOopsieCard.fake()
        };

        let (card_id, board, _) = generate_board_with_open_card(Card::from(oopsie_card));
        let board_with_resourecs = Board {
            current_resources: Resources::new(23),
            cost_modifier: Some(CostModifier::Increase(Resources::new(2))),
            ..board
        };

        let real_fix_costs = Resources::new(24);

        let expected_board = Board {
            current_resources: Resources::new(0),
            cost_modifier: None,
            ..board_with_resourecs.clone()
        };

        let result = try_and_pay_for_oopsie_fix(
            board_with_resourecs,
            &card_id,
            ResourceFixMultiplier::new(2),
        )
        .unwrap_err();

        assert_eq!(
            result,
            ActionError::NotEnoughResources(expected_board, real_fix_costs)
        )
    }
}
