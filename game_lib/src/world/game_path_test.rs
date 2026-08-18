/*
This file contains longer tests for game mechanics.
*/

#[cfg(test)]
mod path_tests {
    use crate::cards::properties::duration::Duration;
    use crate::cards::properties::effect::Effect;
    use crate::cards::properties::effect_description::EffectDescription;
    use crate::cards::properties::incident_impact::IncidentImpact;
    use crate::cards::properties::target::Target;
    use crate::cards::properties::title::Title;
    use crate::cards::types::attack::tests::FakeAttackCard;
    use crate::cards::types::attack::AttackCard;
    use crate::cards::types::card_model::{Card, CardTrait};
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::world::board::Board;
    use crate::world::deck::Deck;
    use crate::world::game::{Game, GameStatus};
    use crate::world::resources::Resources;
    use fake::Fake;
    use uuid::Uuid;

    mod attack_highlighting {
        use super::*;
        use crate::world::game::GameInitSettings;
        use std::rc::Rc;

        const ATTACK_CARD_TITLE: &str = "Attack card";
        const OOPSIE_CARD_TITLE: &str = "Oopsie card";

        /*
        Creates a game with three cards: Oopsie, Attack, and Event
        The Oopsie and Attack cards have matching targets.
         */
        fn create_game() -> Game {
            let oopies_card = OopsieCard {
                title: Title::new(OOPSIE_CARD_TITLE),
                effect: Effect::AttackSurface(
                    EffectDescription::new("Attack surface"),
                    vec![Target::new("network")],
                ),
                ..FakeOopsieCard.fake()
            };

            let attack_card = AttackCard {
                title: Title::new(ATTACK_CARD_TITLE),
                effect: Effect::Incident(
                    EffectDescription::new("Attack surface"),
                    vec![Target::new("network")],
                    IncidentImpact::Fixed(Resources::new(10)),
                ),
                duration: Duration::new(Some(5)),
                ..FakeAttackCard.fake()
            };

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
            assert_eq!(active_incident.attack_title, ATTACK_CARD_TITLE);
            assert_eq!(active_incident.oopsie_title, OOPSIE_CARD_TITLE);
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
                find_card_id_by_title(&board_with_active_incident, ATTACK_CARD_TITLE);

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
                find_card_id_by_title(&board_with_active_incident, OOPSIE_CARD_TITLE);

            let closed_oopsie = active_incident.close_card(oopsie_card_id);
            let board_after_closed_oopsie = get_board_from_game(&closed_oopsie);
            assert_eq!(
                board_after_closed_oopsie.active_incidents.len(),
                0,
                "Oopsie is fixed, no incident expected"
            );
        }
    }

    mod reputation_handling_for_incident {
        use crate::cards::properties::duration::Duration;
        use crate::cards::properties::effect::Effect;
        use crate::cards::properties::effect_description::EffectDescription;
        use crate::cards::properties::incident_impact::IncidentImpact;
        use crate::cards::properties::target::Target;
        use crate::cards::properties::title::Title;
        use crate::cards::types::attack::tests::FakeAttackCard;
        use crate::cards::types::attack::AttackCard;
        use crate::cards::types::card_model::Card;
        use crate::cards::types::oopsie::tests::FakeOopsieCard;
        use crate::cards::types::oopsie::OopsieCard;
        use crate::world::deck::Deck;
        use crate::world::game::{Game, GameInitSettings, ReputationSettings};
        use crate::world::game_path_test::path_tests::get_board_from_game;
        use crate::world::reputation::Reputation;
        use crate::world::resources::Resources;
        use fake::Fake;
        use std::rc::Rc;

        const NETWORK_ATTACK_CARD_TITLE: &str = "Attack card";
        const NETWORK_OOPSIE_CARD_TITLE_1: &str = "Network Oopsie card 1";
        const NETWORK_OOPSIE_CARD_TITLE_2: &str = "Network Oopsie card 2";
        const MISSING_ATTACK_CARD_TITLE: &str = "Mising attack card";

        struct AvailableCards {
            network_oopsie_1: Card,
            network_oopsie_2: Card,
            network_attack: Card,
            missing_attack: Card,
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

            AvailableCards {
                network_oopsie_1: Card::from(network_oopsie_1),
                network_oopsie_2: Card::from(network_oopsie_2),
                network_attack: Card::from(network_attack),
                missing_attack: Card::from(missing_attack),
            }
        }

        fn create_deck(cards: Vec<Card>) -> Deck {
            Deck::new(cards.iter().map(|c| Rc::new(c.clone())).collect())
        }

        mod non_stacked {
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

        mod stacked {
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
