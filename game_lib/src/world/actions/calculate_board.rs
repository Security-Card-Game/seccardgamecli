/*
This action should go over all open cards and calculate fix costs and modifiers.
It has to be called when
- A new card is drawn
- Any card is closed
- Any card is applied
 */
use std::collections::HashSet;

use uuid::Uuid;

use crate::cards::properties::cost_modifier::CostModifier;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::target::Target;
use crate::cards::types::card_model::Card;
use crate::world::board::{Board, Incident};
use crate::world::deck::{CardRc, Deck};
use crate::world::resources::Resources;

pub(crate) fn calculate_board(board: Board, deck: &Deck) -> Board {
    let remaining_rounds = calculate_remaining_rounds(deck);
    let fix_modifier = calculate_cost_modifier(&board);
    let active_incidents = determine_active_incidents(&board);
    Board {
        turns_remaining: remaining_rounds,
        cost_modifier: fix_modifier,
        active_incidents,
        ..board
    }
}

fn determine_active_incidents(board: &Board) -> Vec<Incident> {
    let attacks = board
        .open_cards
        .iter()
        .filter_map(|(id, card)| {
            if let Card::Attack(attack) = &**card {
                Some((id, attack))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let oopsies = board
        .open_cards
        .iter()
        .filter_map(|(id, card)| {
            if let Card::Oopsie(oopsie) = &**card {
                Some((id, oopsie))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut incidents = Vec::new();

    for (attack_id, attack) in attacks.iter() {
        let attack_targets = match &attack.effect {
            Effect::Incident(_, targets, _) => targets,
            _ => continue,
        };

        for (oopsie_id, oopsie) in oopsies.iter() {
            let oopsie_targets = match &oopsie.effect {
                Effect::AttackSurface(_, targets) => targets,
                _ => continue,
            };

            let attack_set: HashSet<_> = HashSet::from_iter(attack_targets.iter());
            let oopsie_set: HashSet<_> = HashSet::from_iter(oopsie_targets.iter());
            let has_matching_target = attack_set.intersection(&oopsie_set).next().is_some();

            if has_matching_target {
                incidents.push(Incident {
                    attack_card_id: **attack_id,
                    attack_title: attack.title.value().to_string(),
                    oopsie_card_id: **oopsie_id,
                    oopsie_title: oopsie.title.value().to_string(),
                });
            }
        }
    }
    incidents
}

fn calculate_cost_modifier(board: &Board) -> Option<CostModifier> {
    let new_modifier = board
        .open_cards
        .iter()
        .filter_map(|(id, card)| get_modifier(id, card, &board.cards_to_use))
        .fold(CostModifier::Decrease(Resources::new(0)), |acc, e| acc + e);

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
) -> Option<CostModifier> {
    match &**card {
        Card::Event(e) => get_modifier_from_effect(&e.effect, cards_to_use.contains(card_id)),
        Card::Attack(_) => None,
        Card::Oopsie(_) => None,
        Card::Lucky(l) => get_modifier_from_effect(&l.effect, cards_to_use.contains(card_id)),
        Card::Evaluation(_) => None,
    }
}

fn get_modifier_from_effect(effect: &Effect, card_is_active: bool) -> Option<CostModifier> {
    match effect {
        Effect::Immediate(_) => None,
        Effect::AttackSurface(_, _) => None,
        Effect::Incident(_, _, _) => None,
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

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::cards::properties::cost_modifier::tests::FakeCostModifier;
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::incident_impact::tests::FakeFixedIncidentImpact;
    use crate::cards::properties::target::Target;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::card_model::Card;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::lucky::tests::FakeLuckyCard;
    use crate::cards::types::lucky::LuckyCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::world::board::Board;
    use crate::world::deck::Deck;
    use fake::Fake;
    use rstest::rstest;
    use uuid::Uuid;
    use crate::cards::properties::title::Title;
    use super::*;

    #[test]
    fn calculate_remaining_rounds() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>()).into();

        let deck = Deck {
            remaining_cards: vec![oopsie_card],
            played_cards: 2,
            total: 3,
        };

        let result = super::calculate_remaining_rounds(&deck);

        assert_eq!(result, 1)
    }

    #[test]
    fn calculate_cost_modifier_from_next_fix_effect() {
        let modifier: CostModifier = FakeCostModifier.fake();
        let effect = Effect::OnNextFix(FakeEffectDescription.fake(), modifier.clone());

        let result = get_modifier_from_effect(&effect, false).unwrap();

        assert_eq!(result, modifier)
    }

    #[test]
    fn calculate_cost_modifier_from_one_user_for_fix_effect_not_active() {
        let modifier: CostModifier = FakeCostModifier.fake();
        let effect = Effect::OnUsingForFix(FakeEffectDescription.fake(), modifier.clone());

        let result = get_modifier_from_effect(&effect, false);

        assert!(result.is_none())
    }

    #[test]
    fn calculate_cost_modifier_from_one_use_for_fix_effect_is_active() {
        let modifier: CostModifier = FakeCostModifier.fake();
        let effect = Effect::OnUsingForFix(FakeEffectDescription.fake(), modifier.clone());

        let result = get_modifier_from_effect(&effect, true).unwrap();

        assert_eq!(result, modifier)
    }

    #[rstest]
    #[case::NOP(Effect::NOP, None)]
    #[case::Immediate(Effect::Immediate(FakeEffectDescription.fake()), None)]
    #[case::AttackSurface(Effect::AttackSurface(FakeEffectDescription.fake(), vec![]), None)]
    #[case::Incident(Effect::Incident(FakeEffectDescription.fake(), vec![], FakeFixedIncidentImpact.fake()), None)]
    #[case::Other(Effect::Other(FakeEffectDescription.fake()), None)]
    fn calculate_fix_modifier_of_non_modifying_effect(
        #[case] effect: Effect,
        #[case] expectation: Option<CostModifier>,
    ) {
        let result = get_modifier_from_effect(&effect, true);

        assert_eq!(result, expectation);
    }

    #[test]
    fn calculate_cost_modifier_for_board() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let oopsie_card_rc = Rc::new(oopsie_card.clone());

        let attack_card = Card::from(FakeAttackCard.fake::<AttackCard>());
        let attack_card_rc = Rc::new(attack_card.clone());

        let event_card_base: EventCard = FakeEventCard.fake();
        let event_modifier: CostModifier = FakeCostModifier.fake();
        let event_card = Card::from(EventCard {
            effect: Effect::OnNextFix(FakeEffectDescription.fake(), event_modifier.clone()),
            ..event_card_base
        });
        let event_card_rc = Rc::new(event_card.clone());

        let used_card_modifier: CostModifier = FakeCostModifier.fake();
        let used_card_id = Uuid::new_v4();
        let used_lucky_card_base: LuckyCard = FakeLuckyCard.fake();
        let used_lucky_card = Card::from(LuckyCard {
            effect: Effect::OnUsingForFix(FakeEffectDescription.fake(), used_card_modifier.clone()),
            ..used_lucky_card_base
        });
        let used_lucky_card_rc = Rc::new(used_lucky_card);

        let unused_lucky_card_base: LuckyCard = FakeLuckyCard.fake();
        let unused_lucky_card = Card::from(LuckyCard {
            effect: Effect::OnUsingForFix(FakeEffectDescription.fake(), FakeCostModifier.fake()),
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

        let result = calculate_cost_modifier(&board);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn calculate_board_no_fix_modifiers() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>()).into();
        let event_card_base: EventCard = FakeEventCard.fake();
        let event_modifier: CostModifier = FakeCostModifier.fake();
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
            cost_modifier: Some(event_modifier),
            ..board.clone()
        };

        let new_board = calculate_board(board, &deck);

        assert_eq!(new_board, expected_board)
    }

    #[test]
    fn determine_active_incidents_empty_board_no_incidents() {
        let empty_board = Board::empty();
        let active_incidents = determine_active_incidents(&empty_board);
        assert_eq!(active_incidents, vec![])
    }

    #[test]
    fn determine_active_incidents_matches_multiple_attacks_one_oopsies() {
        let (uuid_oopsie_backend, oopsie_card_rc_backend) = generate_oopsie(Target::new("backend"), "o1");
        let (uuid_attack_backend_1, attack_card_rc_backend_1) = generate_attack(Target::new("backend"), "a1");
        let (uuid_attack_backend_2, attack_card_rc_backend_2) = generate_attack(Target::new("backend"), "a2");
        let (uuid_attack_frontend, attack_card_rc_frontend) = generate_attack(Target::new("frontend"), "a3");

        let cards = vec![
            (uuid_oopsie_backend.clone(), oopsie_card_rc_backend),
            (uuid_attack_backend_1.clone(), attack_card_rc_backend_1),
            (uuid_attack_backend_2.clone(), attack_card_rc_backend_2),
            (uuid_attack_frontend.clone(), attack_card_rc_frontend),
        ];
        let open_cards: HashMap<_, _> = cards.into_iter().collect();

        let board = Board {
            open_cards,
            ..Board::empty()
        };

        let active_incidents = determine_active_incidents(&board);

        assert_vec_eq_ignore_order(
            active_incidents,
            vec![
                Incident {
                    attack_card_id: uuid_attack_backend_1,
                    attack_title: "a1".to_string(),
                    oopsie_card_id: uuid_oopsie_backend,
                    oopsie_title: "o1".to_string()
                },
                Incident {
                    attack_card_id: uuid_attack_backend_2,
                    attack_title: "a2".to_string(),
                    oopsie_card_id: uuid_oopsie_backend,
                    oopsie_title: "o1".to_string()
                },
            ]
        );
    }

    #[test]
    fn determine_active_incidents_matches_one_attack_multiple_oopsies() {
        let (uuid_oopsie_backend_1, oopsie_card_rc_backend_1) = generate_oopsie(Target::new("backend"), "o1");
        let (uuid_oopsie_backend_2, oopsie_card_rc_backend_2) = generate_oopsie(Target::new("backend"), "o2");
        let (uuid_oopsie_frontend, oopsie_card_rc_frontend) = generate_oopsie(Target::new("fronted"), "o3");
        let (uuid_attack, attack_card_rc) = generate_attack(Target::new("backend"), "a");

        let cards = vec![
            (uuid_oopsie_backend_1.clone(), oopsie_card_rc_backend_1),
            (uuid_oopsie_backend_2.clone(), oopsie_card_rc_backend_2),
            (uuid_oopsie_frontend.clone(), oopsie_card_rc_frontend),
            (uuid_attack.clone(), attack_card_rc),
        ];
        let open_cards: HashMap<_, _> = cards.into_iter().collect();

        let board = Board {
            open_cards,
            ..Board::empty()
        };

        let active_incidents = determine_active_incidents(&board);

        assert_vec_eq_ignore_order(
            active_incidents,
            vec![
                Incident {
                    attack_card_id: uuid_attack,
                    attack_title: "a".to_string(),
                    oopsie_card_id: uuid_oopsie_backend_1,
                    oopsie_title: "o1".to_string()
                },
                Incident {
                    attack_card_id: uuid_attack,
                    attack_title: "a".to_string(),
                    oopsie_card_id: uuid_oopsie_backend_2,
                    oopsie_title: "o2".to_string()
                }
            ]
        );
    }

    fn generate_oopsie(target: Target, title: &str) -> (Uuid, Rc<Card>) {
        (
            Uuid::new_v4(),
            Rc::new(Card::from(OopsieCard {
                effect: Effect::AttackSurface(
                    FakeEffectDescription.fake(),
                    vec![target],
                ),
                title: Title::new(title),
                ..FakeOopsieCard.fake::<OopsieCard>()
            })),
        )
    }

    fn generate_attack(target: Target, title: &str) -> (Uuid, Rc<Card>) {
        (
            Uuid::new_v4(),
            Rc::new(Card::from(AttackCard {
                effect: Effect::Incident(
                    FakeEffectDescription.fake(),
                    vec![target],
                    FakeFixedIncidentImpact.fake(),
                ),
                title: Title::new(title),
                ..FakeAttackCard.fake::<AttackCard>()
            }))
        )
    }

    fn assert_vec_eq_ignore_order<T: Ord + std::fmt::Debug>(mut a: Vec<T>, mut b: Vec<T>) {
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn determine_active_incidents_matches_one_incident_with_attack() {
        let (uuid_oopsie, oopsie_card_rc) = generate_oopsie(Target::new("backend"), "o");
        let (uuid_attack, attack_card_rc) = generate_attack(Target::new("backend"), "a");
        let cards = vec![
            (uuid_oopsie.clone(), oopsie_card_rc),
            (uuid_attack.clone(), attack_card_rc),
        ];
        let open_cards: HashMap<_, _> = cards.into_iter().collect();

        let board = Board {
            open_cards,
            ..Board::empty()
        };

        let active_incidents = determine_active_incidents(&board);

        let expected_incident = Incident {
            attack_card_id: uuid_attack,
            attack_title: "a".to_string(),
            oopsie_card_id: uuid_oopsie,
            oopsie_title: "o".to_string()
        };

        assert_vec_eq_ignore_order(active_incidents, vec![expected_incident])
    }

    #[test]
    fn calculate_board_fix_modifiers() {
        let oopsie_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
        let oopsie_card_rc = Rc::new(oopsie_card.clone());
        let cards = vec![(Uuid::new_v4(), oopsie_card_rc.clone())];
        let open_cards: HashMap<_, _> = cards.into_iter().collect();

        let deck = Deck {
            remaining_cards: vec![oopsie_card_rc],
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
