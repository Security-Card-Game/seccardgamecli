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
    use crate::cards::types::card_model::Card;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::oopsie::tests::FakeOopsieCard;
    use crate::cards::types::oopsie::OopsieCard;
    use crate::world::board::Board;
    use crate::world::deck::Deck;
    use crate::world::game::{Game, GameStatus};
    use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
    use crate::world::resources::Resources;
    use fake::Fake;

    mod attack_highlighting {
        use std::collections::HashMap;
        use uuid::Uuid;
        use crate::cards::types::card_model::CardTrait;
        use super::*;

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
                Card::from(oopies_card.clone()),
                Card::from(attack_card.clone()),
                Card::from(EventCard {
                    ..FakeEventCard.fake()
                }),
            ]);

            Game::create(deck, Resources::new(1000), ResourceFixMultiplier::new(2))
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
            assert_eq!(board_with_active_incident.active_incidents.len(), 1, "Init requirement");

            let attack_card_id = find_card_id_by_title(&board_with_active_incident, ATTACK_CARD_TITLE);

            let closed_attack = active_incident.close_card(attack_card_id);
            let board_after_closed_attack = get_board_from_game(&closed_attack);
            assert_eq!(board_after_closed_attack.active_incidents.len(), 0, "Attack is over, no incident expected");
        }

        #[test]
        fn incident_is_over_if_oopsie_gets_closed() {
            let start_game = create_game();

            let active_incident = start_game.next_round().next_round();
            let board_with_active_incident = get_board_from_game(&active_incident);
            assert_eq!(board_with_active_incident.active_incidents.len(), 1, "Init requirement");

            let oopsie_card_id = find_card_id_by_title(&board_with_active_incident, OOPSIE_CARD_TITLE);

            let closed_oopsie = active_incident.close_card(oopsie_card_id);
            let board_after_closed_oopsie = get_board_from_game(&closed_oopsie);
            assert_eq!(board_after_closed_oopsie.active_incidents.len(), 0, "Oopsie is fixed, no incident expected");
        }

        fn find_card_id_by_title<'a>(board: &'a Board, title: &str) -> &'a Uuid {
            board.open_cards.iter().find(|(_, card) | *&card.title().value() == title).unwrap().0
        }

    }

    fn get_board_from_game(game: &Game) -> Board {
        match &game.status {
            GameStatus::InProgress(b) | GameStatus::Start(b) | GameStatus::Finished(b) => b.clone(),
        }
    }
}
