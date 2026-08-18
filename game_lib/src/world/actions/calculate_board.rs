/*
This action should go over all open cards and calculate fix costs and modifiers.
It has to be called when
- A new card is drawn
- Any card is closed
- Any card is applied
 */
use log::warn;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::cards::properties::cost_modifier::CostModifier;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::incident_impact::IncidentImpact;
use crate::cards::types::card_model::Card;
use crate::world::board::{Board, Incident, ResourceEffect};
use crate::world::deck::{CardRc, Deck};
use crate::world::game::ReputationSettings;
use crate::world::part_of_hundred::PartOfHundred;
use crate::world::reputation::Reputation;
use crate::world::resources::Resources;

pub(crate) fn progress_board_to_next_turn(
    board: Board,
    deck: &Deck,
    reputation_settings: &ReputationSettings,
    force_set_resource_gain: &Option<Resources>,
) -> Board {
    let previous_active_incidents = &board.active_incidents.clone();
    let update_board = calculate_board(board, deck, force_set_resource_gain);
    let active_incidents = determine_active_incidents(&update_board);
    let reputation_decrease = calculate_reputation_decrease(
        &previous_active_incidents,
        &active_incidents,
        &reputation_settings,
    );

    let reputation_gain =
        calculate_reputation_gain(&update_board, &active_incidents, &reputation_settings);
    let current_reputation =
        update_board.current_reputation + reputation_gain.bonus + reputation_gain.turn_based
            - reputation_decrease;

    Board {
        incident_free_turns: calculate_incident_free_turns(&update_board, &active_incidents),
        current_reputation,
        current_resources: &update_board.current_resources + &update_board.resource_gain,
        ..update_board
    }
}

pub(crate) fn calculate_board(
    board: Board,
    deck: &Deck,
    force_set_resource_gain: &Option<Resources>,
) -> Board {
    let remaining_rounds = calculate_remaining_rounds(deck);
    let fix_modifier = calculate_cost_modifier(&board);
    let active_incidents = determine_active_incidents(&board);

    let resource_gain = if let Some(manual_gain) = force_set_resource_gain {
        (manual_gain.clone(), board.active_incident_resource_effects)
    } else {
        calculate_incident_resource_effects(&board, &active_incidents)
    };

    Board {
        turns_remaining: remaining_rounds,
        cost_modifier: fix_modifier,
        resource_gain: resource_gain.0.clone(),
        active_incident_resource_effects: resource_gain.1,
        active_incidents,
        ..board
    }
}

fn calculate_incident_resource_effects(
    previous_board: &Board,
    current_incidents: &Vec<Incident>,
) -> (Resources, Vec<ResourceEffect>) {
    let previous_active_incidents = &previous_board
        .active_incidents
        .iter()
        .map(|i| i.attack_card_id)
        .collect::<HashSet<_>>();
    let current_active_incidents = &current_incidents
        .iter()
        .map(|i| i.attack_card_id)
        .collect::<HashSet<_>>();

    let new_incidents = current_active_incidents
        .iter()
        .filter(|i| !previous_active_incidents.contains(i))
        .collect::<Vec<_>>();
    if new_incidents.len() > 1 {
        warn!("More then one new incident!");
    }

    let resolved_incidents = previous_active_incidents
        .iter()
        .filter(|i| !current_active_incidents.contains(i))
        .collect::<Vec<_>>();

    // this feels a bit strange, but open cards are set before calculation of effects
    let open_cards = previous_board.open_cards.clone();
    let mut new_effects = previous_board.active_incident_resource_effects.clone();

    let amount_to_reduce =
        add_new_incident_effects(&previous_board, new_incidents, open_cards, &mut new_effects);
    let amount_to_increase =
        reverse_resolved_incident_effects(resolved_incidents, &mut new_effects);

    (
        &previous_board.resource_gain - &amount_to_reduce + amount_to_increase,
        new_effects,
    )
}

fn add_new_incident_effects(
    board: &Board,
    new_incidents: Vec<&Uuid>,
    open_cards: HashMap<Uuid, CardRc>,
    new_effects: &mut Vec<ResourceEffect>,
) -> Resources {
    let mut amount_to_reduce = Resources::new(0);
    for incident in new_incidents {
        let effect = if let Some(card) = open_cards.get(incident) {
            match &**card {
                Card::Attack(a) => match &a.effect {
                    Effect::Incident(_, _, e) => match e {
                        IncidentImpact::PartOfRevenue(p) => {
                            calculate_relative_resource_effect(board, incident, p)
                        }
                        IncidentImpact::Fixed(f) => {
                            calculate_fixed_resource_effect(board, incident, f)
                        }
                    },
                    _ => {
                        warn!("No incident effect!");
                        None
                    }
                },
                _ => {
                    warn!("No attack card!");
                    None
                }
            }
        } else {
            None
        };
        if let Some(e) = effect {
            amount_to_reduce = amount_to_reduce + e.effect;
            new_effects.push(e)
        };
    }
    amount_to_reduce
}

fn calculate_fixed_resource_effect(
    board: &Board,
    incident: &Uuid,
    fixed_amount: &Resources,
) -> Option<ResourceEffect> {
    Some(ResourceEffect {
        attack_card_id: incident.clone(),
        effect: min(board.resource_gain, fixed_amount.clone()),
    })
}

fn calculate_relative_resource_effect(
    board: &Board,
    incident: &Uuid,
    p: &PartOfHundred,
) -> Option<ResourceEffect> {
    let calculated = board.resource_gain.value().clone() as f32 * (p.value as f32) / 100f32;
    let effect = min(
        board.resource_gain,
        Resources::new(calculated.round() as usize),
    );
    Some(ResourceEffect {
        attack_card_id: incident.clone(),
        effect,
    })
}

fn reverse_resolved_incident_effects(
    finished_incidents: Vec<&Uuid>,
    new_effects: &mut Vec<ResourceEffect>,
) -> Resources {
    let mut amount_to_increase = Resources::new(0);
    for resolved_incident in finished_incidents {
        let idx_resolved_effect = new_effects
            .iter()
            .position(|x| &x.attack_card_id == resolved_incident);
        if let Some(idx) = idx_resolved_effect {
            let effect = new_effects.remove(idx);
            amount_to_increase = amount_to_increase + effect.effect;
        } else {
            warn!(
                "No effect found for resolved incident {}",
                resolved_incident
            );
        }
    }
    amount_to_increase
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ReputationGain {
    bonus: Reputation,
    turn_based: Reputation,
}

fn calculate_reputation_gain(
    previous_board: &Board,
    current_incidents: &Vec<Incident>,
    reputation_settings: &ReputationSettings,
) -> ReputationGain {
    if !(current_incidents.is_empty()) {
        return ReputationGain {
            bonus: Reputation::new(0),
            turn_based: Reputation::new(0),
        };
    }

    if reputation_settings.gain_active {
        let turn_based = if previous_board.incident_free_turns
            > reputation_settings.gain_incident_free_turns as usize
        {
            reputation_settings.gain_turn_based
        } else {
            Reputation::new(0)
        };

        let bonus = if previous_board.incident_free_turns
            == reputation_settings.gain_incident_free_turns as usize
        {
            reputation_settings.gain_bonus
        } else {
            Reputation::new(0)
        };

        ReputationGain { bonus, turn_based }
    } else {
        ReputationGain {
            bonus: Reputation::new(0),
            turn_based: Reputation::new(0),
        }
    }
}

fn calculate_incident_free_turns(board: &Board, active_incidents: &Vec<Incident>) -> usize {
    if active_incidents.is_empty() {
        board.incident_free_turns + 1
    } else {
        0
    }
}

fn calculate_reputation_decrease(
    previous_incidents: &Vec<Incident>,
    current_incidents: &Vec<Incident>,
    reputation_settings: &ReputationSettings,
) -> Reputation {
    if reputation_settings.incident_penalty_stacked {
        calculate_stacked_reputation_decrease(
            previous_incidents,
            current_incidents,
            reputation_settings,
        )
    } else {
        calculate_non_stacked_reputation_decrease(
            previous_incidents,
            current_incidents,
            reputation_settings,
        )
    }
}

fn calculate_non_stacked_reputation_decrease(
    previous_incidents: &Vec<Incident>,
    current_incidents: &Vec<Incident>,
    reputation_settings: &ReputationSettings,
) -> Reputation {
    let previous = previous_incidents
        .iter()
        .map(|i| i.attack_card_id)
        .collect::<HashSet<_>>();
    let current = current_incidents
        .iter()
        .map(|i| i.attack_card_id)
        .collect::<HashSet<_>>();

    // get values in current but not in previous
    let new_incidents_count = current.difference(&previous).count() as u8;
    Reputation::new(new_incidents_count * reputation_settings.incident_penalty.value())
}

fn calculate_stacked_reputation_decrease(
    previous_incidents: &Vec<Incident>,
    current_incidents: &Vec<Incident>,
    reputation_settings: &ReputationSettings,
) -> Reputation {
    let previous = group_oopsies_by_incident(previous_incidents);
    let current = group_oopsies_by_incident(current_incidents);

    let mut count: u8 = 0;

    for incident in current {
        let old_incident_oopsies = previous.get(&incident.0);
        count += if let Some(old_oopsies) =
            old_incident_oopsies.map(|i| HashSet::from_iter(i.iter().cloned()))
        {
            let current_oopsies: HashSet<Uuid> = HashSet::from_iter(incident.1.iter().cloned());
            current_oopsies.difference(&old_oopsies).count() as u8
        } else {
            incident.1.len() as u8
        }
    }

    Reputation::new(count * reputation_settings.incident_penalty.value())
}

fn group_oopsies_by_incident(incidents: &Vec<Incident>) -> HashMap<Uuid, Vec<Uuid>> {
    let mut groups: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for incident in incidents {
        groups
            .entry(incident.attack_card_id)
            .or_default()
            .push(incident.oopsie_card_id);
    }
    groups
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

    use super::*;
    use crate::cards::properties::cost_modifier::tests::FakeCostModifier;
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::incident_impact::tests::FakeFixedIncidentImpact;
    use crate::cards::properties::target::Target;
    use crate::cards::properties::title::Title;
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
            incident_free_turns: 1,
            cost_modifier: Some(event_modifier),
            ..board.clone()
        };

        let new_board =
            progress_board_to_next_turn(board, &deck, &ReputationSettings::default(), &None);

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
        let (uuid_oopsie_backend, oopsie_card_rc_backend) =
            generate_oopsie(Target::new("backend"), "o1");
        let (uuid_attack_backend_1, attack_card_rc_backend_1) =
            generate_attack(Target::new("backend"), "a1");
        let (uuid_attack_backend_2, attack_card_rc_backend_2) =
            generate_attack(Target::new("backend"), "a2");
        let (uuid_attack_frontend, attack_card_rc_frontend) =
            generate_attack(Target::new("frontend"), "a3");

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
                    oopsie_title: "o1".to_string(),
                },
                Incident {
                    attack_card_id: uuid_attack_backend_2,
                    attack_title: "a2".to_string(),
                    oopsie_card_id: uuid_oopsie_backend,
                    oopsie_title: "o1".to_string(),
                },
            ],
        );
    }

    #[test]
    fn determine_active_incidents_matches_one_attack_multiple_oopsies() {
        let (uuid_oopsie_backend_1, oopsie_card_rc_backend_1) =
            generate_oopsie(Target::new("backend"), "o1");
        let (uuid_oopsie_backend_2, oopsie_card_rc_backend_2) =
            generate_oopsie(Target::new("backend"), "o2");
        let (uuid_oopsie_frontend, oopsie_card_rc_frontend) =
            generate_oopsie(Target::new("fronted"), "o3");
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
                    oopsie_title: "o1".to_string(),
                },
                Incident {
                    attack_card_id: uuid_attack,
                    attack_title: "a".to_string(),
                    oopsie_card_id: uuid_oopsie_backend_2,
                    oopsie_title: "o2".to_string(),
                },
            ],
        );
    }

    fn generate_oopsie(target: Target, title: &str) -> (Uuid, Rc<Card>) {
        (
            Uuid::new_v4(),
            Rc::new(Card::from(OopsieCard {
                effect: Effect::AttackSurface(FakeEffectDescription.fake(), vec![target]),
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
            })),
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
            oopsie_title: "o".to_string(),
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
            incident_free_turns: 1,
            ..board.clone()
        };

        let new_board =
            progress_board_to_next_turn(board, &deck, &ReputationSettings::default(), &None);

        assert_eq!(new_board, expected_board)
    }

    mod reputation {
        use crate::world::actions::calculate_board::calculate_reputation_decrease;
        use crate::world::board::Incident;
        use crate::world::game::ReputationSettings;
        use crate::world::reputation::Reputation;
        use uuid::Uuid;
        mod non_stacking {
            use super::*;
            #[test]
            fn no_incidents_no_decrease() {
                let settings = ReputationSettings::default();
                let previous_incidents = Vec::new();
                let current_incidents = Vec::new();
                let expected_decrease = Reputation::new(0);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn one_new_incidents_one_time_decrease() {
                let settings = ReputationSettings::default();
                let previous_incidents = Vec::new();
                let current_incidents = vec![Incident {
                    attack_card_id: Uuid::new_v4(),
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let expected_decrease = settings.incident_penalty;

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn one_new_incidents_multiple_oopsies_one_time_decrease() {
                let settings = ReputationSettings::default();
                let previous_incidents = Vec::new();
                let attack_id = Uuid::new_v4();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: attack_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: attack_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Another Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = settings.incident_penalty;

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn two_new_incidents_different_oopsies_two_times_decrease() {
                let settings = ReputationSettings::default();
                let previous_incidents = Vec::new();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Another Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = settings.incident_penalty.multiply(2);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn decrease_overflows_reputation_max() {
                let settings = ReputationSettings {
                    incident_penalty: Reputation::new(99),
                    ..ReputationSettings::default()
                };
                let previous_incidents = Vec::new();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Another Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = Reputation::new(100);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn two_incidents_with_same_oopsie_reduces_twice() {
                let settings = ReputationSettings::default();
                let previous_incidents = Vec::new();
                let oopsie_id = Uuid::new_v4();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: oopsie_id,
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: oopsie_id,
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = settings.incident_penalty.multiply(2);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn no_new_incident_no_decrease() {
                let settings = ReputationSettings::default();
                let previous_incidents = vec![Incident {
                    attack_card_id: Uuid::new_v4(),
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let current_incidents = previous_incidents.clone();
                let expected_decrease = Reputation::new(0);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn no_new_incident_but_additions_oopsie_no_decrease() {
                let settings = ReputationSettings::default();
                let attack_card_id = Uuid::new_v4();

                let previous_incidents = vec![Incident {
                    attack_card_id,
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let mut current_incidents = previous_incidents.clone();
                current_incidents.append(&mut vec![Incident {
                    attack_card_id,
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }]);
                let expected_decrease = Reputation::new(0);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }
        }

        mod stacking {
            use super::*;

            fn stacked_default_settings() -> ReputationSettings {
                ReputationSettings {
                    incident_penalty_stacked: true,
                    ..ReputationSettings::default()
                }
            }
            #[test]
            fn no_incidents_no_decrease() {
                let settings = stacked_default_settings();
                let previous_incidents = Vec::new();
                let current_incidents = Vec::new();
                let expected_decrease = Reputation::new(0);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn one_new_incidents_with_one_oopsie_one_time_decrease() {
                let settings = stacked_default_settings();
                let previous_incidents = Vec::new();
                let current_incidents = vec![Incident {
                    attack_card_id: Uuid::new_v4(),
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let expected_decrease = settings.incident_penalty;

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn one_new_incidents_with_two_oopsies_two_time_decrease() {
                let settings = stacked_default_settings();
                let previous_incidents = Vec::new();
                let attack_id = Uuid::new_v4();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: attack_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: attack_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Another Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = settings.incident_penalty.multiply(2);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn two_new_incidents_different_oopsies_two_times_decrease() {
                let settings = stacked_default_settings();
                let previous_incidents = Vec::new();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Another Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = settings.incident_penalty.multiply(2);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn decrease_overflows_reputation_max() {
                let settings = ReputationSettings {
                    incident_penalty: Reputation::new(99),
                    ..stacked_default_settings()
                };
                let previous_incidents = Vec::new();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Another Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = Reputation::new(100);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn two_incidents_with_same_oopsie_reduces_twice() {
                let settings = stacked_default_settings();
                let previous_incidents = Vec::new();
                let oopsie_id = Uuid::new_v4();
                let current_incidents = vec![
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: oopsie_id,
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: oopsie_id,
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                ];
                let expected_decrease = settings.incident_penalty.multiply(2);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn no_new_incident_no_decrease() {
                let settings = stacked_default_settings();
                let previous_incidents = vec![Incident {
                    attack_card_id: Uuid::new_v4(),
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let current_incidents = previous_incidents.clone();
                let expected_decrease = Reputation::new(0);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn no_new_incident_but_additional_oopsies_additional_decrease() {
                let settings = stacked_default_settings();
                let attack_card_id = Uuid::new_v4();

                let previous_incidents = vec![Incident {
                    attack_card_id,
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let mut current_incidents = previous_incidents.clone();
                current_incidents.append(&mut vec![
                    Incident {
                        attack_card_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                ]);
                let expected_decrease = settings.incident_penalty.multiply(2);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }

            #[test]
            fn new_incident_and_additional_oopsie_multiple_time_decrease() {
                let settings = stacked_default_settings();
                let attack_card_id = Uuid::new_v4();
                let oopsie_card_id = Uuid::new_v4();
                let previous_incidents = vec![Incident {
                    attack_card_id,
                    attack_title: "Attack Title".to_string(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: "Oopsie Title".to_string(),
                }];
                let mut current_incidents = previous_incidents.clone();
                current_incidents.append(&mut vec![
                    Incident {
                        attack_card_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: Uuid::new_v4(),
                        oopsie_title: "Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id,
                        attack_title: "Attack Title".to_string(),
                        oopsie_card_id: oopsie_card_id.clone(),
                        oopsie_title: "Shared Oopsie Title".to_string(),
                    },
                    Incident {
                        attack_card_id: Uuid::new_v4(),
                        attack_title: "New Attack Title".to_string(),
                        oopsie_card_id: oopsie_card_id.clone(),
                        oopsie_title: "Shared Oopsie Title".to_string(),
                    },
                ]);
                let expected_decrease = settings.incident_penalty.multiply(3);

                let reputation_decrease = calculate_reputation_decrease(
                    &previous_incidents,
                    &current_incidents,
                    &settings,
                );

                assert_eq!(reputation_decrease, expected_decrease);
            }
        }
    }

    mod incident_free_effects {
        use super::*;
        use crate::world::board::Board;
        use uuid::Uuid;

        fn create_incident_vec() -> Vec<Incident> {
            vec![Incident {
                attack_card_id: Uuid::new_v4(),
                attack_title: "New Attack Title".to_string(),
                oopsie_card_id: Uuid::new_v4(),
                oopsie_title: "Shared Oopsie Title".to_string(),
            }]
        }

        mod reputation_gain {
            use super::*;

            #[test]
            fn no_resource_gain_when_gain_deactive() {
                let board_for_bonus = Board {
                    incident_free_turns: ReputationSettings::default().gain_incident_free_turns
                        as usize,
                    ..Board::empty()
                };
                let board_for_turn = Board {
                    incident_free_turns: ReputationSettings::default().gain_incident_free_turns
                        as usize
                        + 1,
                    ..Board::empty()
                };
                let settings = ReputationSettings {
                    gain_active: false,
                    ..ReputationSettings::default()
                };

                let incidents = Vec::new();

                let reputation_gain_bonus =
                    calculate_reputation_gain(&board_for_bonus, &incidents, &settings);
                let reputation_gain_turn =
                    calculate_reputation_gain(&board_for_turn, &incidents, &settings);

                assert_eq!(
                    reputation_gain_bonus,
                    ReputationGain {
                        turn_based: Reputation::new(0),
                        bonus: Reputation::new(0)
                    }
                );

                assert_eq!(
                    reputation_gain_turn,
                    ReputationGain {
                        turn_based: Reputation::new(0),
                        bonus: Reputation::new(0)
                    }
                );
            }

            #[test]
            fn no_reputation_bonus_and_no_gain_below_threshold() {
                let settings = ReputationSettings::default();
                let board = Board {
                    incident_free_turns: settings.gain_incident_free_turns as usize - 2,
                    ..Board::empty()
                };
                let incidents = Vec::new();
                let expected_result = ReputationGain {
                    bonus: Reputation::new(0),
                    turn_based: Reputation::new(0),
                };

                let result = calculate_reputation_gain(&board, &incidents, &settings);

                assert_eq!(result, expected_result);
            }

            #[test]
            fn reputation_bonus_but_no_gain_at_threshold() {
                let settings = ReputationSettings::default();
                let board = Board {
                    incident_free_turns: settings.gain_incident_free_turns as usize,
                    ..Board::empty()
                };
                let incidents = Vec::new();
                let expected_result = ReputationGain {
                    bonus: settings.gain_bonus,
                    turn_based: Reputation::new(0),
                };

                let result = calculate_reputation_gain(&board, &incidents, &settings);

                assert_eq!(result, expected_result);
            }

            #[test]
            fn no_reputation_bonus_but_reputation_gain_above_threshold() {
                let settings = ReputationSettings::default();
                let board = Board {
                    incident_free_turns: settings.gain_incident_free_turns as usize + 1,
                    ..Board::empty()
                };
                let incidents = Vec::new();
                let expected_result = ReputationGain {
                    bonus: Reputation::new(0),
                    turn_based: settings.gain_turn_based,
                };

                let result = calculate_reputation_gain(&board, &incidents, &settings);

                assert_eq!(result, expected_result);
            }

            #[test]
            fn no_reputation_bonus_and_gain_on_incident() {
                let settings = ReputationSettings::default();
                let board = Board {
                    incident_free_turns: settings.gain_incident_free_turns as usize + 1,
                    ..Board::empty()
                };
                let incidents = create_incident_vec();
                let expected_result = ReputationGain {
                    bonus: Reputation::new(0),
                    turn_based: Reputation::new(0),
                };

                let result = calculate_reputation_gain(&board, &incidents, &settings);

                assert_eq!(result, expected_result);
            }
        }
    }

    mod resource_effects {
        use super::*;
        use crate::cards::properties::duration::Duration;
        use crate::cards::properties::effect_description::EffectDescription;
        use std::iter::zip;

        fn create_attack_card(impact: IncidentImpact) -> Card {
            let attack_card = AttackCard {
                title: Title::new("Incident"),
                effect: Effect::Incident(
                    EffectDescription::new("Relative Incident"),
                    vec![Target::new("network")],
                    impact,
                ),
                duration: Duration::new(Some(5)),
                ..FakeAttackCard.fake::<AttackCard>()
            };
            Card::from(attack_card)
        }

        #[test]
        fn fixed_value_incident_returns_fixed_value_when_lower_then_resource_gain() {
            let incident_id = Uuid::new_v4();
            let board = Board {
                resource_gain: Resources::new(10),
                ..Board::empty()
            };
            let expected_result = Some(ResourceEffect {
                attack_card_id: incident_id,
                effect: Resources::new(9),
            });

            let result = calculate_fixed_resource_effect(&board, &incident_id, &Resources::new(9));

            assert_eq!(result, expected_result);
        }

        #[test]
        fn fixed_value_incident_returns_resource_gain__when_fixed_effect_higher_then_resource_gain()
        {
            let incident_id = Uuid::new_v4();
            let board = Board {
                resource_gain: Resources::new(10),
                ..Board::empty()
            };
            let expected_result = Some(ResourceEffect {
                attack_card_id: incident_id,
                effect: Resources::new(10),
            });

            let result = calculate_fixed_resource_effect(&board, &incident_id, &Resources::new(11));

            assert_eq!(result, expected_result);
        }

        #[test]
        fn relative_value_incident_returns_rounded_results_when_lower_then_resource_gain() {
            let incident_id = Uuid::new_v4();
            let board = Board {
                resource_gain: Resources::new(10),
                ..Board::empty()
            };
            let expected_result_rounded_up = Some(ResourceEffect {
                attack_card_id: incident_id,
                effect: Resources::new(4),
            });

            let expected_result = Some(ResourceEffect {
                attack_card_id: incident_id,
                effect: Resources::new(5),
            });

            let expected_result_rounded_down = Some(ResourceEffect {
                attack_card_id: incident_id,
                effect: Resources::new(6),
            });

            let result_rounded_up =
                calculate_relative_resource_effect(&board, &incident_id, &PartOfHundred::new(44));
            assert_eq!(result_rounded_up, expected_result_rounded_up);

            let result =
                calculate_relative_resource_effect(&board, &incident_id, &PartOfHundred::new(50));
            assert_eq!(result, expected_result);

            let result_rounded_down =
                calculate_relative_resource_effect(&board, &incident_id, &PartOfHundred::new(55));
            assert_eq!(result_rounded_down, expected_result_rounded_down);
        }

        #[test]
        fn relative_value_incident_returns_max_resource_gain() {
            let incident_id = Uuid::new_v4();
            let board = Board {
                resource_gain: Resources::new(10),
                ..Board::empty()
            };
            let expected_result = Some(ResourceEffect {
                attack_card_id: incident_id,
                effect: Resources::new(10),
            });

            let result =
                calculate_relative_resource_effect(&board, &incident_id, &PartOfHundred::new(100));
            assert_eq!(result, expected_result);
        }

        #[test]
        fn resolved_incident_reverses_effect() {
            let effects = vec![
                ResourceEffect {
                    attack_card_id: Uuid::new_v4(),
                    effect: Resources::new(10),
                },
                ResourceEffect {
                    attack_card_id: Uuid::new_v4(),
                    effect: Resources::new(5),
                },
                ResourceEffect {
                    attack_card_id: Uuid::new_v4(),
                    effect: Resources::new(3),
                },
            ];
            let mut new_effects = effects.clone();
            let resolved_incident_id = effects[1..=2]
                .iter()
                .map(|effect| &effect.attack_card_id)
                .collect::<Vec<&Uuid>>();
            let resources_to_add_to_gain =
                reverse_resolved_incident_effects(resolved_incident_id, &mut new_effects);
            assert_eq!(resources_to_add_to_gain, Resources::new(8));
            assert_eq!(new_effects.len(), 1);
            assert_eq!(new_effects[0].effect, Resources::new(10));
        }

        #[test]
        fn calculate_only_new_incident_resource_effects() {
            let cards = vec![
                Card::from(create_attack_card(IncidentImpact::Fixed(Resources::new(5)))),
                Card::from(create_attack_card(IncidentImpact::PartOfRevenue(
                    PartOfHundred::new(50),
                ))),
                Card::from(create_attack_card(IncidentImpact::Fixed(Resources::new(4)))),
            ];
            let incident_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

            let open_cards = zip(incident_ids.iter(), cards.iter())
                .map(|(id, card)| (*id, CardRc::new(card.clone())))
                .collect::<HashMap<Uuid, CardRc>>();

            let active_incidents_before = incident_ids[0..2]
                .iter()
                .map(|i| Incident {
                    attack_card_id: *i,
                    attack_title: String::new(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: String::new(),
                })
                .collect::<Vec<Incident>>();

            let active_effects_before = vec![
                ResourceEffect {
                    attack_card_id: incident_ids[0],
                    effect: Resources::new(5),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[1],
                    effect: Resources::new(3),
                },
            ];

            let previous_board = Board {
                open_cards,
                active_incidents: active_incidents_before,
                active_incident_resource_effects: active_effects_before,
                resource_gain: Resources::new(10),
                ..Board::empty()
            };

            let active_incidents = incident_ids
                .iter()
                .map(|i| Incident {
                    attack_card_id: *i,
                    attack_title: String::new(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: String::new(),
                })
                .collect::<Vec<Incident>>();

            let expected_new_gain = Resources::new(6);
            let expected_effects = vec![
                ResourceEffect {
                    attack_card_id: incident_ids[0],
                    effect: Resources::new(5),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[1],
                    effect: Resources::new(3),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[2],
                    effect: Resources::new(4),
                },
            ];

            let result = calculate_incident_resource_effects(&previous_board, &active_incidents);
            let new_gain = result.0;
            let active_effects = result.1;

            assert_eq!(new_gain, expected_new_gain);
            assert_eq!(active_effects, expected_effects);
        }

        #[test]
        fn calculate_only_resolved_incident_resource_effects() {
            let cards = vec![
                Card::from(create_attack_card(IncidentImpact::Fixed(Resources::new(5)))),
                Card::from(create_attack_card(IncidentImpact::PartOfRevenue(
                    PartOfHundred::new(50),
                ))),
                Card::from(create_attack_card(IncidentImpact::Fixed(Resources::new(4)))),
            ];
            let incident_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

            let open_cards = zip(incident_ids.iter(), cards.iter())
                .map(|(id, card)| (*id, CardRc::new(card.clone())))
                .collect::<HashMap<Uuid, CardRc>>();

            let active_incidents_before = incident_ids
                .iter()
                .map(|i| Incident {
                    attack_card_id: *i,
                    attack_title: String::new(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: String::new(),
                })
                .collect::<Vec<Incident>>();

            let active_effects_before = vec![
                ResourceEffect {
                    attack_card_id: incident_ids[0],
                    effect: Resources::new(5),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[1],
                    effect: Resources::new(3),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[2],
                    effect: Resources::new(4),
                },
            ];

            let previous_board = Board {
                open_cards,
                active_incidents: active_incidents_before,
                active_incident_resource_effects: active_effects_before,
                resource_gain: Resources::new(10),
                ..Board::empty()
            };

            let active_incidents = vec![Incident {
                attack_card_id: incident_ids[2],
                attack_title: String::new(),
                oopsie_card_id: Uuid::new_v4(),
                oopsie_title: String::new(),
            }];
            // two incidents are resolved 5 + 3 + 10 current gain
            let expected_new_gain = Resources::new(18);
            let expected_effects = vec![ResourceEffect {
                attack_card_id: incident_ids[2],
                effect: Resources::new(4),
            }];

            let result = calculate_incident_resource_effects(&previous_board, &active_incidents);
            let new_gain = result.0;
            let active_effects = result.1;

            assert_eq!(new_gain, expected_new_gain);
            assert_eq!(active_effects, expected_effects);
        }

        #[test]
        fn calculate_mixture_incident_resource_effects() {
            let cards = vec![
                Card::from(create_attack_card(IncidentImpact::Fixed(Resources::new(5)))),
                Card::from(create_attack_card(IncidentImpact::PartOfRevenue(
                    PartOfHundred::new(50),
                ))),
                Card::from(create_attack_card(IncidentImpact::Fixed(Resources::new(4)))),
            ];
            let incident_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

            let open_cards = zip(incident_ids.iter(), cards.iter())
                .map(|(id, card)| (*id, CardRc::new(card.clone())))
                .collect::<HashMap<Uuid, CardRc>>();

            let active_incidents_before = incident_ids[0..=1]
                .iter()
                .map(|i| Incident {
                    attack_card_id: *i,
                    attack_title: String::new(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: String::new(),
                })
                .collect::<Vec<Incident>>();

            let active_effects_before = vec![
                ResourceEffect {
                    attack_card_id: incident_ids[0],
                    effect: Resources::new(5),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[1],
                    effect: Resources::new(3),
                },
            ];

            let previous_board = Board {
                open_cards,
                active_incidents: active_incidents_before,
                active_incident_resource_effects: active_effects_before,
                resource_gain: Resources::new(10),
                ..Board::empty()
            };

            let active_incidents = vec![
                Incident {
                    attack_card_id: incident_ids[1],
                    attack_title: String::new(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: String::new(),
                },
                Incident {
                    attack_card_id: incident_ids[2],
                    attack_title: String::new(),
                    oopsie_card_id: Uuid::new_v4(),
                    oopsie_title: String::new(),
                },
            ];

            // one incident is resolved +5, one is new -4: 10 + 5 -4
            let expected_new_gain = Resources::new(11);
            let expected_effects = vec![
                ResourceEffect {
                    attack_card_id: incident_ids[1],
                    effect: Resources::new(3),
                },
                ResourceEffect {
                    attack_card_id: incident_ids[2],
                    effect: Resources::new(4),
                },
            ];

            let result = calculate_incident_resource_effects(&previous_board, &active_incidents);
            let new_gain = result.0;
            let active_effects = result.1;

            assert_eq!(new_gain, expected_new_gain);
            assert_eq!(active_effects, expected_effects);
        }
    }
}
