/*
This action should go over all open cards and calculate fix costs and modifiers.
It has to be called when
- A new card is drawn
- Any card is closed
- Any card is applied
 */
use std::collections::HashSet;

use uuid::Uuid;

use crate::cards::properties::effect::Effect;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::card_model::Card;
use crate::world::board::Board;
use crate::world::deck::{CardRc, Deck};
use crate::world::resources::Resources;


pub(crate) fn calculate_board(board: Board, deck: &Deck) -> Board {
    let remaining_rounds = calculate_remaining_rounds(deck);
    let fix_modifier = calculate_fix_modifier(&board);
    Board {
        turns_remaining: remaining_rounds,
        fix_modifier,
        ..board
    }
}

fn calculate_fix_modifier(board: &Board) -> Option<FixModifier> {
    let new_modifier = board
        .open_cards
        .iter()
        .filter_map(|(id, card)| get_modifier(id, card, &board.cards_to_use))
        .fold(FixModifier::Decrease(Resources::new(0)), |acc, e| acc + e);

    if new_modifier.value() == 0 {
        None
    } else {
        Some(new_modifier.clone())
    }
}

fn get_modifier(
    card_id: &Uuid,
    card: &CardRc,
    cards_to_use: &HashSet<Uuid>,
) -> Option<FixModifier> {
    match &**card {
        Card::Event(e) => get_modifier_from_effect(&e.effect, cards_to_use.contains(card_id)),
        Card::Attack(_) => None,
        Card::Oopsie(_) => None,
        Card::Lucky(l) => get_modifier_from_effect(&l.effect, cards_to_use.contains(card_id)),
        Card::Evaluation(_) => None,
    }
}

fn get_modifier_from_effect(effect: &Effect, card_is_active: bool) -> Option<FixModifier> {
    match effect {
        Effect::Immediate(_) => None,
        Effect::AttackSurface(_, _) => None,
        Effect::Incident(_, _) => None,
        Effect::OnNextFix(_, m) => Some(m.clone()),
        Effect::OnUsingForFix(_, m) => {
            if card_is_active {
                Some(m.clone())
            } else {
                None
            }
        }
        Effect::Other(_) => None,
        Effect::NOP => None,
    }
}

fn calculate_remaining_rounds(deck: &Deck) -> usize {
    deck.get_remaining_card_count()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;

    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::fix_modifier::tests::FakeFixModifier;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::card_model::Card;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::lucky::LuckyCard;
    use crate::cards::types::lucky::tests::FakeLuckyCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::world::board::Board;
    use crate::world::deck::Deck;

    use super::*;

    #[test]
    fn calculate_remaining_rounds() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());

        let deck = Deck {
            remaining_cards: vec![oopsie_card],
            played_cards: 2,
            total: 3,
        };

        let result = super::calculate_remaining_rounds(&deck);

        assert_eq!(result, 1)
    }

    #[test]
    fn calculate_fix_modifier_from_next_fix_effect() {
        let modifier: FixModifier = FakeFixModifier.fake();
        let effect = Effect::OnNextFix(FakeEffectDescription.fake(), modifier.clone());

        let result = get_modifier_from_effect(&effect, false).unwrap();

        assert_eq!(result, modifier)
    }

    #[test]
    fn calculate_fix_modifier_from_one_user_for_fix_effect_not_active() {
        let modifier: FixModifier = FakeFixModifier.fake();
        let effect = Effect::OnUsingForFix(FakeEffectDescription.fake(), modifier.clone());

        let result = get_modifier_from_effect(&effect, false);

        assert!(result.is_none())
    }

    #[test]
    fn calculate_fix_modifier_from_one_use_for_fix_effect_is_active() {
        let modifier: FixModifier = FakeFixModifier.fake();
        let effect = Effect::OnUsingForFix(FakeEffectDescription.fake(), modifier.clone());

        let result = get_modifier_from_effect(&effect, true).unwrap();

        assert_eq!(result, modifier)
    }

    #[rstest]
    #[case::NOP(Effect::NOP, None)]
    #[case::Immediate(Effect::Immediate(FakeEffectDescription.fake()), None)]
    #[case::AttackSurface(Effect::AttackSurface(FakeEffectDescription.fake(), vec![]), None)]
    #[case::Incident(Effect::Incident(FakeEffectDescription.fake(), vec![]), None)]
    #[case::Other(Effect::Other(FakeEffectDescription.fake()), None)]
    fn calculate_fix_modifier_of_non_modifying_effect(
        #[case] effect: Effect,
        #[case] expectation: Option<FixModifier>,
    ) {
        let result = get_modifier_from_effect(&effect, true);

        assert_eq!(result, expectation);
    }

    #[test]
    fn calculate_fix_modifier_for_board() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let oopsie_card_rc = Rc::new(oopsie_card.clone());

        let attack_card = Card::from(FakeAttackCard.fake::<AttackCard>());
        let attack_card_rc = Rc::new(attack_card.clone());

        let event_card_base: EventCard = FakeEventCard.fake();
        let event_modifier: FixModifier = FakeFixModifier.fake();
        let event_card = Card::from(EventCard {
            effect: Effect::OnNextFix(FakeEffectDescription.fake(), event_modifier.clone()),
            ..event_card_base
        });
        let event_card_rc = Rc::new(event_card.clone());

        let used_card_modifier: FixModifier = FakeFixModifier.fake();
        let used_card_id = Uuid::new_v4();
        let used_lucky_card_base: LuckyCard = FakeLuckyCard.fake();
        let used_lucky_card = Card::from(LuckyCard {
            effect: Effect::OnUsingForFix(FakeEffectDescription.fake(), used_card_modifier.clone()),
            ..used_lucky_card_base
        });
        let used_lucky_card_rc = Rc::new(used_lucky_card);

        let unused_lucky_card_base: LuckyCard = FakeLuckyCard.fake();
        let unused_lucky_card = Card::from(LuckyCard {
            effect: Effect::OnUsingForFix(FakeEffectDescription.fake(), FakeFixModifier.fake()),
            ..unused_lucky_card_base
        });
        let unused_lucky_card_rc = Rc::new(unused_lucky_card);

        let cards = vec![
            (Uuid::new_v4(), oopsie_card_rc),
            (Uuid::new_v4(), event_card_rc),
            (Uuid::new_v4(), attack_card_rc),
            (Uuid::new_v4(), unused_lucky_card_rc),
            (used_card_id.clone(), used_lucky_card_rc),
        ];

        let open_cards: HashMap<_, _> = cards.into_iter().collect();
        let cards_to_use = &mut HashSet::new();
        cards_to_use.insert(used_card_id);

        let board = Board {
            open_cards,
            cards_to_use: cards_to_use.clone(),
            ..Board::empty()
        };

        let test_result = used_card_modifier + event_modifier;
        let expected_result = if test_result.value() == 0 {
            None
        } else {
            Some(test_result)
        };

        let result = calculate_fix_modifier(&board);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn calculate_board_no_fix_modifiers() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let event_card_base: EventCard = FakeEventCard.fake();
        let event_modifier: FixModifier = FakeFixModifier.fake();
        let event_card = Card::from(EventCard {
            effect: Effect::OnNextFix(FakeEffectDescription.fake(), event_modifier.clone()),
            ..event_card_base
        });
        let event_card_rc = Rc::new(event_card.clone());

        let cards = vec![(Uuid::new_v4(), event_card_rc)];
        let open_cards: HashMap<_, _> = cards.into_iter().collect();

        let deck = Deck {
            remaining_cards: vec![oopsie_card],
            played_cards: 2,
            total: 3,
        };

        let board = Board {
            open_cards,
            ..Board::empty()
        };

        let expected_board = Board {
            turns_remaining: 1,
            fix_modifier: Some(event_modifier),
            ..board.clone()
        };

        let new_board = calculate_board(board, &deck);

        assert_eq!(new_board, expected_board)
    }

    #[test]
    fn calculate_board_fix_modifiers() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let oopsie_card_rc = Rc::new(oopsie_card.clone());
        let cards = vec![(Uuid::new_v4(), oopsie_card_rc)];
        let open_cards: HashMap<_, _> = cards.into_iter().collect();

        let deck = Deck {
            remaining_cards: vec![oopsie_card],
            played_cards: 2,
            total: 3,
        };

        let board = Board {
            open_cards,
            ..Board::empty()
        };

        let expected_board = Board {
            turns_remaining: 1,
            ..board.clone()
        };

        let new_board = calculate_board(board, &deck);

        assert_eq!(new_board, expected_board)
    }
}
