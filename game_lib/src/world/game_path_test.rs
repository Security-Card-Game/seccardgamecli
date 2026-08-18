/*
This file contains longer tests for game mechanics.
*/

#[cfg(test)]
mod path_tests {
    use std::rc::Rc;
    use crate::cards::properties::duration::Duration;
    use crate::cards::properties::effect::Effect;
    use crate::cards::properties::effect_description::EffectDescription;
    use crate::cards::properties::incident_impact::IncidentImpact;
    use crate::cards::properties::target::Target;
    use crate::cards::properties::title::Title;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::card_model::{Card, CardTrait};
    use crate::cards::types::event::tests::{FakeEventCard, FakeNoOpEventCard};
    use crate::cards::types::event::EventCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::world::board::Board;
    use crate::world::deck::Deck;
    use crate::world::game::{Game, GameStatus};
    use crate::world::resources::Resources;
    use fake::Fake;
    use uuid::Uuid;

    const NETWORK_ATTACK_CARD_TITLE: &str = "Attack card";
    const NETWORK_OOPSIE_CARD_TITLE_1: &str = "Network Oopsie card 1";
    const NETWORK_OOPSIE_CARD_TITLE_2: &str = "Network Oopsie card 2";
    const MISSING_ATTACK_CARD_TITLE: &str = "Mising attack card";

    struct AvailableCards {
        network_oopsie_1: Card,
        network_oopsie_2: Card,
        network_attack: Card,
        missing_attack: Card,
        no_op_cards: Vec<Card>,
    }

    fn available_cards() -> AvailableCards {
        let network_oopsie_1 = OopsieCard {
            title: Title::new(NETWORK_OOPSIE_CARD_TITLE_1),
            effect: Effect::AttackSurface(
                EffectDescription::new("Attack surface"),
                vec![Target::new("network")],
            ),
            ..FakeOopsieCard.fake()
        };

        let network_oopsie_2 = OopsieCard {
            title: Title::new(NETWORK_OOPSIE_CARD_TITLE_2),
            effect: Effect::AttackSurface(
                EffectDescription::new("Attack surface"),
                vec![Target::new("network")],
            ),
            ..FakeOopsieCard.fake()
        };

        let network_attack = AttackCard {
            title: Title::new(NETWORK_ATTACK_CARD_TITLE),
            effect: Effect::Incident(
                EffectDescription::new("Attack surface"),
                vec![Target::new("network")],
                IncidentImpact::Fixed(Resources::new(10)),
            ),
            duration: Duration::new(Some(5)),
            ..FakeAttackCard.fake()
        };

        let missing_attack = AttackCard {
            title: Title::new(MISSING_ATTACK_CARD_TITLE),
            effect: Effect::Incident(
                EffectDescription::new("Attack surface"),
                vec![Target::new("none")],
                IncidentImpact::Fixed(Resources::new(10)),
            ),
            duration: Duration::new(Some(5)),
            ..FakeAttackCard.fake()
        };

        let no_op_cards = (1..20).into_iter()
            .map(|_| Card::from(FakeNoOpEventCard.fake::<EventCard>())).collect();

        AvailableCards {
            network_oopsie_1: Card::from(network_oopsie_1),
            network_oopsie_2: Card::from(network_oopsie_2),
            network_attack: Card::from(network_attack),
            missing_attack: Card::from(missing_attack),
            no_op_cards
        }
    }

    fn create_deck(cards: Vec<Card>) -> Deck {
        Deck::new(cards.iter().map(|c| Rc::new(c.clone())).collect())
    }


    mod attack_highlighting {
        use super::*;
        use crate::world::game::GameInitSettings;
        use std::rc::Rc;
        /*
        Creates a game with three cards: Oopsie, Attack, and Event
        The Oopsie and Attack cards have matching targets.
         */
        fn create_game() -> Game {
            let available_cards = available_cards();
            let oopies_card = available_cards.network_oopsie_1;
            let attack_card = available_cards.network_attack;

            let deck = Deck::new(vec![
                Rc::new(Card::from(oopies_card.clone())),
                Rc::new(Card::from(attack_card.clone())),
                Rc::new(Card::from(EventCard {
                    ..FakeEventCard.fake()
                })),
            ]);

            let init_settings = GameInitSettings {
                resources: Resources::new(100),
                ..GameInitSettings::default()
            };

            Game::create(deck, init_settings)
        }

        #[test]
        fn incident_is_active_if_attack_matches_oopsie() {
            let start_game = create_game();

            let oopsie_drawn = start_game.next_round();
            let board_with_oopsie: Board = get_board_from_game(&oopsie_drawn);
            assert_eq!(board_with_oopsie.active_incidents.len(), 0);

            let attack_drawn = oopsie_drawn.next_round();
            let board_with_attack_and_oopsie = get_board_from_game(&attack_drawn);
            assert_eq!(
                board_with_attack_and_oopsie.active_incidents.len(),
                1,
                "open_cards: {:?}",
                board_with_attack_and_oopsie.open_cards.len()
            );

            let active_incident = board_with_attack_and_oopsie.active_incidents[0].clone();
            assert_eq!(active_incident.attack_title, NETWORK_ATTACK_CARD_TITLE);
            assert_eq!(active_incident.oopsie_title, NETWORK_OOPSIE_CARD_TITLE_1);
        }

        #[test]
        fn incident_is_over_if_attack_gets_closed() {
            let start_game = create_game();

            let active_incident = start_game.next_round().next_round();
            let board_with_active_incident = get_board_from_game(&active_incident);
            assert_eq!(
                board_with_active_incident.active_incidents.len(),
                1,
                "Init requirement"
            );

            let attack_card_id =
                find_card_id_by_title(&board_with_active_incident, NETWORK_ATTACK_CARD_TITLE);

            let closed_attack = active_incident.close_card(attack_card_id);
            let board_after_closed_attack = get_board_from_game(&closed_attack);
            assert_eq!(
                board_after_closed_attack.active_incidents.len(),
                0,
                "Attack is over, no incident expected"
            );
        }

        #[test]
        fn incident_is_over_if_oopsie_gets_closed() {
            let start_game = create_game();

            let active_incident = start_game.next_round().next_round();
            let board_with_active_incident = get_board_from_game(&active_incident);
            assert_eq!(
                board_with_active_incident.active_incidents.len(),
                1,
                "Init requirement"
            );

            let oopsie_card_id =
                find_card_id_by_title(&board_with_active_incident, NETWORK_OOPSIE_CARD_TITLE_1);

            let closed_oopsie = active_incident.close_card(oopsie_card_id);
            let board_after_closed_oopsie = get_board_from_game(&closed_oopsie);
            assert_eq!(
                board_after_closed_oopsie.active_incidents.len(),
                0,
                "Oopsie is fixed, no incident expected"
            );
        }
    }

    mod reputation {
        use super::*;
        use crate::cards::types::card_model::Card;
        use crate::world::deck::Deck;
        use crate::world::game::{Game, GameInitSettings, ReputationSettings};
        use crate::world::game_path_test::path_tests::get_board_from_game;
        use crate::world::reputation::Reputation;
        use crate::world::resources::Resources;

        mod incident_non_stacked {
            use crate::world::game_path_test::path_tests::{available_cards, create_deck};
            use super::*;
            fn create_game(deck: Deck) -> Game {
                let init_settings = GameInitSettings {
                    resources: Resources::new(100),
                    ..GameInitSettings::default()
                };

                Game::create(deck, init_settings)
            }

            fn create_game_with_incident_penalty(deck: Deck, incident_penalty: Reputation) -> Game {
                let init_settings = GameInitSettings {
                    resources: Resources::new(100),
                    reputation: ReputationSettings {
                        incident_penalty,
                        ..ReputationSettings::default()
                    },
                    ..GameInitSettings::default()
                };

                Game::create(deck, init_settings)
            }

            #[test]
            fn no_incident_no_change() {
                let available_cards = available_cards();
                let deck = create_deck(vec![
                    available_cards.network_oopsie_1,
                    available_cards.missing_attack,
                ]);
                let game = create_game(deck);
                let oopsie_drawn = game.next_round();
                let initial_reputation = get_board_from_game(&oopsie_drawn)
                    .current_reputation
                    .clone();

                let attack_drawn = oopsie_drawn.next_round();
                let attack_reputation = get_board_from_game(&attack_drawn)
                    .current_reputation
                    .clone();

                assert_eq!(initial_reputation, attack_reputation, "Attack was expected to become an incident and reduce reputation, before attack was {}, after attack was {}", initial_reputation, attack_reputation)
            }

            #[test]
            fn attack_ended_no_incident_no_change() {
                let available_cards = available_cards();
                let deck = create_deck(vec![
                    available_cards.network_attack,
                    available_cards.network_oopsie_1,
                ]);
                let game = create_game(deck);
                let attack_drawn = game.next_round();
                let initial_reputation = get_board_from_game(&attack_drawn)
                    .current_reputation
                    .clone();
                let attack_closed =
                    attack_drawn.close_card(&get_board_from_game(&attack_drawn).drawn_card.unwrap().id);

                let oopsie_drawn = attack_closed.next_round();
                let attack_reputation = get_board_from_game(&oopsie_drawn)
                    .current_reputation
                    .clone();

                assert_eq!(initial_reputation, attack_reputation, "Attack was expected to become an incident and reduce reputation, before attack was {}, after attack was {}", initial_reputation, attack_reputation)
            }

            #[test]
            fn incident_reputation_decreases() {
                let available_cards = available_cards();
                let deck = create_deck(vec![
                    available_cards.network_oopsie_1,
                    available_cards.network_attack,
                ]);
                let game = create_game(deck);
                let oopsie_drawn = game.next_round();
                let initial_reputation = get_board_from_game(&oopsie_drawn)
                    .current_reputation
                    .clone();

                let attack_drawn = oopsie_drawn.next_round();
                let attack_reputation = get_board_from_game(&attack_drawn)
                    .current_reputation
                    .clone();

                assert_ne!(initial_reputation, attack_reputation, "Attack was expected to become an incident and reduce reputation, before attack was {}, after attack was {}", initial_reputation, attack_reputation);
                let expected_reputation = &initial_reputation - &Reputation::new(5);
                assert_eq!(
                    attack_reputation, expected_reputation,
                    "Expected reputation to decrease to {}, was {}",
                    expected_reputation, attack_reputation
                )
            }

            #[test]
            fn ongoing_attack_becomes_incident_reputation_decreases() {
                let available_cards = available_cards();
                let deck = create_deck(vec![
                    available_cards.network_attack,
                    available_cards.network_oopsie_1,
                ]);
                let game = create_game(deck);
                let oopsie_drawn = game.next_round();
                let initial_reputation = get_board_from_game(&oopsie_drawn)
                    .current_reputation
                    .clone();

                let attack_drawn = oopsie_drawn.next_round();
                let attack_reputation = get_board_from_game(&attack_drawn)
                    .current_reputation
                    .clone();

                assert_ne!(initial_reputation, attack_reputation, "Attack was expected to become an incident and reduce reputation, before attack was {}, after attack was {}", initial_reputation, attack_reputation);
                let expected_reputation = &initial_reputation - &Reputation::new(5);
                assert_eq!(
                    attack_reputation, expected_reputation,
                    "Expected reputation to decrease to {}, was {}",
                    expected_reputation, attack_reputation
                )
            }

            #[test]
            fn reputation_settings_incident_penalty_is_used() {
                let available_cards = available_cards();
                let custom_penalty = Reputation::new(15);

                let deck = create_deck(vec![
                    available_cards.network_attack,
                    available_cards.network_oopsie_1,
                ]);
                let game = create_game_with_incident_penalty(deck, custom_penalty.clone());
                let oopsie_drawn = game.next_round();
                let initial_reputation = get_board_from_game(&oopsie_drawn)
                    .current_reputation
                    .clone();

                let attack_drawn = oopsie_drawn.next_round();
                let attack_reputation = get_board_from_game(&attack_drawn)
                    .current_reputation
                    .clone();

                assert_ne!(initial_reputation, attack_reputation, "Attack was expected to become an incident and reduce reputation, before attack was {}, after attack was {}", initial_reputation, attack_reputation);
                let expected_reputation = &initial_reputation - &custom_penalty;
                assert_eq!(
                    attack_reputation, expected_reputation,
                    "Expected reputation to decrease to {}, was {}",
                    expected_reputation, attack_reputation
                )
            }
        }

        mod incident_stacked {
            use super::*;

            fn create_game(deck: Deck) -> Game {
                let init_settings = GameInitSettings {
                    resources: Resources::new(100),
                    reputation: ReputationSettings {
                        incident_penalty_stacked: true,
                        ..ReputationSettings::default()
                    },
                    ..GameInitSettings::default()
                };

                Game::create(deck, init_settings)
            }

            fn create_game_with_incident_penalty(deck: Deck, incident_penalty: Reputation) -> Game {
                let init_settings = GameInitSettings {
                    resources: Resources::new(100),
                    reputation: ReputationSettings {
                        incident_penalty,
                        incident_penalty_stacked: true,
                        ..ReputationSettings::default()
                    },
                    ..GameInitSettings::default()
                };

                Game::create(deck, init_settings)
            }

            #[test]
            fn stacked_penalty_is_used() {
                let deck = create_deck(
                    vec![available_cards().network_oopsie_1, available_cards().network_oopsie_2, available_cards().network_attack]
                );

                let game = create_game(deck);

                let oopsie_1_drawn = game.next_round();
                let oopsie_2_drawn = oopsie_1_drawn.next_round();
                let base_reputation = get_board_from_game(&oopsie_2_drawn).current_reputation.clone();

                let attack_drawn = oopsie_2_drawn.next_round();
                let incident_reputation = get_board_from_game(&attack_drawn).current_reputation.clone();

                let expected_reputation = &base_reputation - &ReputationSettings::default().incident_penalty.multiply(2);

                assert_eq!(incident_reputation, expected_reputation, "Expected penalty to be stacked twice")
            }

            #[test]
            fn stacked_penalty_with_custom_penalty_is_used() {
                let deck = create_deck(
                    vec![available_cards().network_oopsie_1, available_cards().network_oopsie_2, available_cards().network_attack]
                );

                let game = create_game_with_incident_penalty(deck, Reputation::new(1));

                let oopsie_1_drawn = game.next_round();
                let oopsie_2_drawn = oopsie_1_drawn.next_round();
                let base_reputation = get_board_from_game(&oopsie_2_drawn).current_reputation.clone();

                let attack_drawn = oopsie_2_drawn.next_round();
                let incident_reputation = get_board_from_game(&attack_drawn).current_reputation.clone();

                let expected_reputation = &base_reputation - &Reputation::new(1).multiply(2);

                assert_eq!(incident_reputation, expected_reputation, "Expected penalty to be stacked twice")
            }

        }

        mod gain  {
            use super::*;
            use crate::world::deck::Deck;
            use crate::world::game::{Game, GameInitSettings, ReputationSettings};
            use crate::world::resources::Resources;

            fn create_game(deck: Deck) -> Game {
                let init_settings = GameInitSettings {
                    resources: Resources::new(100),
                    ..GameInitSettings::default()
                };

                Game::create(deck, init_settings)
            }

            fn create_game_with_gain_deactive(deck: Deck) -> Game {
                let init_settings = GameInitSettings {
                    resources: Resources::new(100),
                    reputation: ReputationSettings {
                        gain_active: false,
                        ..ReputationSettings::default()
                    },
                    ..GameInitSettings::default()
                };

                Game::create(deck, init_settings)
            }

            fn default_reputation_bonus() -> Reputation {
                ReputationSettings::default().gain_bonus
            }

            fn default_turn_based_reputation_gain() -> Reputation {
                ReputationSettings::default().gain_turn_based
            }

            fn default_incident_free_turns() -> u8 {
                ReputationSettings::default().gain_incident_free_turns
            }

            #[test]
            fn no_incidents_give_bonus_and_round_based_gain() {
                let available_cards = available_cards();
                let mut cards = vec![available_cards.network_oopsie_1];
                cards.append(&mut available_cards.no_op_cards.clone());
                let deck = create_deck(cards);
                let initial_game = create_game(deck);
                let initial_board = get_board_from_game(&initial_game);
                let initial_reputation = initial_board.current_reputation;

                let mut game_before_bonus = initial_game.clone();
                for _ in 1..=default_incident_free_turns() {
                   game_before_bonus = game_before_bonus.next_round()
                }

                let game_before_bonus_reputation = get_board_from_game(&game_before_bonus).current_reputation;

                assert_eq!(initial_reputation, game_before_bonus_reputation, "No Reputation change expected");

                // activate bonus
                let game_after_bonus = game_before_bonus.next_round();
                let game_after_bonus_reputation = get_board_from_game(&game_after_bonus).current_reputation;
                let expected_reputation_after_bonus = &initial_reputation + &default_reputation_bonus();

                assert_eq!(game_after_bonus_reputation, expected_reputation_after_bonus, "Bonus expected");

                // play n more rounds and check turn based reputation gain
                let mut game_turn_based_gain = game_after_bonus.clone();
                let rounds_to_play: u8 = 5;
                let expected_reputation_after_turns =  &expected_reputation_after_bonus + &default_turn_based_reputation_gain().multiply(rounds_to_play);
                for _ in 1..=rounds_to_play {
                    game_turn_based_gain = game_turn_based_gain.next_round()
                }
                let game_after_turn_based_gain_reputation = get_board_from_game(&game_turn_based_gain).current_reputation;

                assert_eq!(game_after_turn_based_gain_reputation, expected_reputation_after_turns, "Turn based gain expected")
            }

            fn get_attack_duration(card: &Card) -> u8 {
                match card {
                    Card::Attack(a) => a.duration.value().unwrap_or(&0).clone() as u8,
                    _ => panic!("No attack!")
                }
            }

            #[test]
            fn incidents_gives_no_bonus_and_gain_continue_without_incident_till_bonus() {
                let available_cards = available_cards();
                let attack = available_cards.network_attack.clone();
                let mut cards = vec![available_cards.network_oopsie_1];
                cards.append(&mut (available_cards.no_op_cards.clone()[1..=3].to_vec()));
                cards.append(&mut vec![attack.clone()]);
                cards.append(&mut available_cards.no_op_cards.clone());
                let deck = create_deck(cards);
                let initial_game = create_game(deck);
                let initial_board = get_board_from_game(&initial_game);
                let initial_reputation = initial_board.current_reputation;

                let mut game_before_bonus = initial_game.clone();
                for _ in 1..default_incident_free_turns() {
                    game_before_bonus = game_before_bonus.next_round();
                }

                let game_before_bonus_reputation = get_board_from_game(&game_before_bonus).current_reputation;

                assert_eq!(initial_reputation, game_before_bonus_reputation, "No Reputation change expected");

                // draw attack that results in incident
                let game_after_incident = game_before_bonus.next_round();
                let game_after_incident_reputation = get_board_from_game(&game_after_incident).current_reputation;
                let expected_reputation_after_incident = initial_reputation - ReputationSettings::default().incident_penalty;

                assert_eq!(game_after_incident_reputation, expected_reputation_after_incident, "Incident penalty expected");

                // play n more rounds and check if bonus activates (detects off by one error)
                let mut game_bonus_gain = game_after_incident.clone();
                let rounds_to_play: u8 = get_attack_duration(&attack) + default_incident_free_turns();
                let expected_reputation_after_bonus =  &expected_reputation_after_incident + &default_reputation_bonus();
                // draw enough cards to activate bonus in next turn
                for _ in 1..rounds_to_play {
                    game_bonus_gain = game_bonus_gain.next_round();
                    let current_reputation = get_board_from_game(&game_bonus_gain).current_reputation;
                    assert_eq!(current_reputation, expected_reputation_after_incident);
                }

                game_bonus_gain = game_bonus_gain.next_round();
                let game_after_bonus_activates_reputation = get_board_from_game(&game_bonus_gain).current_reputation;

                assert_eq!(game_after_bonus_activates_reputation, expected_reputation_after_bonus, "Bonus expected")
            }

            #[test]
            fn gain_deactive_no_incidents_no_reputation_change() {
                let available_cards = available_cards();
                let mut cards = vec![available_cards.network_oopsie_1];
                cards.append(&mut available_cards.no_op_cards.clone());
                let deck = create_deck(cards);
                let initial_game = create_game_with_gain_deactive(deck);
                let initial_board = get_board_from_game(&initial_game);
                let initial_reputation = initial_board.current_reputation;

                let mut game = initial_game.clone();
                for _ in 1..(initial_board.incident_free_turns + 10) {
                    game = game.next_round()
                }

                let game_reputation = get_board_from_game(&game).current_reputation;

                assert_eq!(initial_reputation, game_reputation, "No Reputation change expected");
            }
        }
    }

    mod resources {
        use fake::Fake;
        use crate::cards::properties::duration::Duration;
        use crate::cards::properties::effect::Effect;
        use crate::cards::properties::effect_description::EffectDescription;
        use crate::cards::properties::incident_impact::IncidentImpact;
        use crate::cards::properties::target::Target;
        use crate::cards::properties::title::Title;
        use crate::cards::types::attack::AttackCard;
        use crate::cards::types::attack::tests::FakeAttackCard;
        use crate::world::deck::Deck;
        use crate::world::game::{Game, GameInitSettings};
        use crate::world::part_of_hundred::PartOfHundred;
        use crate::world::resources::Resources;

        fn create_fixed_incident_effect(amount: Resources) -> AttackCard {
            AttackCard {
                title: Title::new("Fixed Incident"),
                effect: Effect::Incident(
                    EffectDescription::new("Fixed Incident"),
                    vec![Target::new("network")],
                    IncidentImpact::Fixed(amount),
                ),
                duration: Duration::new(Some(5)),
                ..FakeAttackCard.fake::<AttackCard>()
            }
        }

        fn create_relative_incident_effect(part_of_hundred: u8) -> AttackCard {
            AttackCard {
                title: Title::new("Relative Incident"),
                effect: Effect::Incident(
                    EffectDescription::new("Relative Incident"),
                    vec![Target::new("network")],
                    IncidentImpact::PartOfRevenue(PartOfHundred::new(part_of_hundred)),
                ),
                duration: Duration::new(Some(5)),
                ..FakeAttackCard.fake::<AttackCard>()
            }
        }

        fn create_game(deck: Deck, resource_gain: Resources) -> Game {
            let init_settings = GameInitSettings {
                resource_gain,
                resources: Resources::new(100),
                ..GameInitSettings::default()
            };

            Game::create(deck, init_settings)

        }

        mod incidents {
            use crate::cards::types::card_model::Card;
            use super::*;
            use crate::world::game::GameStatus;
            use crate::world::game_path_test::path_tests::{available_cards, create_deck};
            use crate::world::game_path_test::path_tests::resources::create_game;
            use crate::world::resources::Resources;

            impl GameStatus {
                fn is_finished(&self) -> bool {
                    match &self {
                        GameStatus::Finished(_) => true,
                        _ => false,
                    }
                }

                fn is_not_finished(&self) -> bool {
                    !self.is_finished()
                }
            }

            #[test]
            fn no_incident_no_changed_resource_gain() {
                let cards = available_cards();
                let deck = create_deck(
                    vec![available_cards().network_oopsie_1, available_cards().missing_attack, available_cards().no_op_cards[0].clone(), available_cards().no_op_cards[1].clone()],
                );
                let card_count = deck.total;
                let resource_gain = Resources::new(10);

                let mut game = create_game(deck, resource_gain);
                while game.status.is_not_finished() {
                    game = game.next_round();
                    assert_eq!(game.resource_gain, resource_gain);
                }
            }

            #[test]
            fn incident_changed_resource_gain_and_reverts_when_done() {
                let cards = available_cards();
                let fixed_incident_1 = create_fixed_incident_effect(Resources::new(5));
                let relative_incident_1 = create_relative_incident_effect(50);
                let fixed_incident_2 = create_fixed_incident_effect(Resources::new(10));
                let relative_incident_2 = create_relative_incident_effect(50);

                let mut cards = vec![available_cards().network_oopsie_1, Card::from(fixed_incident_1), Card::from(relative_incident_1), Card::from(relative_incident_2), Card::from(fixed_incident_2)];
                cards.append(&mut available_cards().no_op_cards.clone());

                let deck = create_deck(cards);
                let initial_resource_gain = Resources::new(20);

                let initial_game = create_game(deck, initial_resource_gain);

                let oopsie_drawn = initial_game.next_round();
                assert_eq!(oopsie_drawn.resource_gain, initial_resource_gain);

                let incident_1 = oopsie_drawn.next_round();
                // -5, dur 5
                assert_eq!(incident_1.resource_gain, Resources::new(15), "Expected fixed effect of -5");

                let incident_2 = incident_1.next_round();
                // -8
                assert_eq!(incident_2.resource_gain, Resources::new(7), "Expected relative effect of 50% of 7.5 -> rounded to 8");

                let incident_3 = incident_2.next_round();
                // -5
                assert_eq!(incident_3.resource_gain, Resources::new(2), "75% of 7 = 5.25 -> 2");

                let incident_4 = incident_3.next_round();
                // -2
                assert_eq!(incident_4.resource_gain, Resources::new(0), "No negative gain");

                let no_change = incident_4.next_round();
                assert_eq!(no_change.resource_gain, Resources::new(0));

                let incident_1_over = no_change.next_round();
                // +5
                assert_eq!(incident_1_over.resource_gain, Resources::new(5));

                let incident_2_over = incident_1_over.next_round();
                // +8
                assert_eq!(incident_2_over.resource_gain, Resources::new(13));

                let incident_3_over = incident_2_over.next_round();
                // +5
                assert_eq!(incident_3_over.resource_gain, Resources::new(18));

                let incident_4_over = incident_3_over.next_round();
                // +2
                assert_eq!(incident_4_over.resource_gain, Resources::new(20));
            }


        }
    }

    fn get_board_from_game(game: &Game) -> Board {
        match &game.status {
            GameStatus::InProgress(b) | GameStatus::Start(b) | GameStatus::Finished(b) => b.clone(),
        }
    }

    fn find_card_id_by_title<'a>(board: &'a Board, title: &str) -> &'a Uuid {
        board
            .open_cards
            .iter()
            .find(|(_, card)| *&card.title().value() == title)
            .unwrap()
            .0
    }
}
