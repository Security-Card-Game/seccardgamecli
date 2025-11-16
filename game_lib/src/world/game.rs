use std::collections::HashMap;

use uuid::Uuid;

use crate::cards::properties::cost_modifier::CostModifier;
use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::{ActionError, ActionResult};
use crate::world::actions::add_reputation::add_reputation;
use crate::world::actions::add_resources::add_resources;
use crate::world::actions::calculate_board::calculate_board;
use crate::world::actions::close_attack::{manually_close_attack_card, update_attack_cards};
use crate::world::actions::close_evaluation::close_evaluation_card;
use crate::world::actions::close_event::close_event_card;
use crate::world::actions::close_lucky::close_lucky_card;
use crate::world::actions::close_oopsie::try_and_pay_for_oopsie_fix;
use crate::world::actions::draw_card::draw_card_and_place_on_board;
use crate::world::actions::subtract_reputation::subtract_reputation;
use crate::world::actions::subtract_resources::subtract_resources;
use crate::world::actions::use_lucky_card::{activate_lucky_card, deactivate_lucky_card};
use crate::world::board::Board;
use crate::world::deck::{CardRc, Deck};
use crate::world::game::GameActionResult::{FixFailed, InvalidAction, OopsieFixed};
use crate::world::reputation::Reputation;
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

/// The central file to interact with the game rules. All external consumers should only use content
/// from in here to play a game.
#[derive(Debug, Clone, PartialEq)]
pub enum GameStatus {
    Start(Board),
    InProgress(Board),
    Finished(Board),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameActionResult {
    Payed,
    NotEnoughResources,
    NothingPayed,
    OopsieFixed(Resources),
    FixFailed(Resources),
    AttackForceClosed,
    InvalidAction,
    Success,
}

/// Game parameters as well as current state and the deck. Every change to the game will return a new game object.
/// Use the action_status property to find out what the outcome of the last action was.
/// The deck is private as it should not be visible to the consumer when a game is played.
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    deck: Deck,
    pub status: GameStatus,
    /// this property contains the result of the last action performed by the player. Use this to know what happened.
    pub action_status: GameActionResult,
    pub resource_gain: Resources,
    pub fix_multiplier: ResourceFixMultiplier,
}

pub struct CardCount {
    pub played_cards: usize,
    pub total_cards: usize,
}

/// This defines the API on how to interact with the Game. It will in turn use corresponding
/// actions from the actions module, combines them when needed. Every interaction return a new Game
/// object.
impl Game {
    /// Gets the currently open cards for the board.
    pub fn get_open_cards(&self) -> HashMap<Uuid, CardRc> {
        self.get_board().open_cards.clone()
    }

    /// Sets a fix multiplier. All effects affection fix costs will be affected by this.
    /// This can be used to modify the cost add hoc in a game.
    pub fn set_fix_multiplier(&self, resource_fix_multiplier: ResourceFixMultiplier) -> Game {
        Game {
            fix_multiplier: resource_fix_multiplier,
            ..self.clone()
        }
    }

    /// Marks a lucky card as activated and re calculates the board to take effects of this card into account.
    pub fn activate_lucky_card(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match activate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::InProgress(calculate_board(new_board, &self.deck)),
                        action_status: GameActionResult::Success,
                        ..self.clone()
                    },
                    Err(_) => Game {
                        action_status: InvalidAction,
                        ..self.clone()
                    },
                }
            }
            GameStatus::Finished(_) => Game {
                action_status: InvalidAction,
                ..self.clone()
            },
        }
    }

    /// Deactivates a lucky card and removes its effect from the board.
    pub fn deactivate_lucky_card(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match deactivate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::InProgress(calculate_board(new_board, &self.deck)),
                        action_status: GameActionResult::Success,
                        ..self.clone()
                    },
                    Err(_) => Game {
                        action_status: InvalidAction,
                        ..self.clone()
                    },
                }
            }
            GameStatus::Finished(_) => Game {
                action_status: InvalidAction,
                ..self.clone()
            },
        }
    }

    pub fn get_current_fix_modifier(&self) -> Option<CostModifier> {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) | GameStatus::Finished(b) => {
                b.cost_modifier.clone()
            }
        }
    }

    fn get_board(&self) -> &Board {
        match self.status {
            GameStatus::Start(ref board)
            | GameStatus::InProgress(ref board)
            | GameStatus::Finished(ref board) => board,
        }
    }

    /// Creates a new game with the given Deck, initial resource gain and fix multiplier.
    /// Use this to start.
    pub fn create(
        deck: Deck,
        initial_resource_gain: Resources,
        fix_multiplier: ResourceFixMultiplier,
    ) -> Self {
        let board = Board::init(&deck, Resources::new(0), Reputation::start_value());
        let status = GameStatus::Start(calculate_board(board, &deck));

        Game {
            deck,
            status,
            action_status: GameActionResult::Success,
            resource_gain: initial_resource_gain.clone(),
            fix_multiplier,
        }
    }

    /// Starts the next round. This also draws a new card and places it on the board and adds
    /// the freshly gained resources.
    pub fn next_round(&self) -> Self {
        if let Ok((new_deck, board)) =
            draw_card_and_place_on_board(self.deck.clone(), self.get_board().clone())
        {
            let board_with_added_resources = add_resources(board, &self.resource_gain);
            let updated_attacks_board = update_attack_cards(board_with_added_resources);
            let new_board = calculate_board(updated_attacks_board, &new_deck);

            let status = if new_board.turns_remaining == 0 {
                GameStatus::Finished(new_board)
            } else {
                GameStatus::InProgress(new_board)
            };
            Game {
                action_status: GameActionResult::Success,
                deck: new_deck,
                status,
                ..self.clone()
            }
        } else {
            Game {
                action_status: InvalidAction,
                ..self.clone()
            }
        }
    }

    /// Manually set the resource gain for the next round.
    pub fn set_resource_gain(&self, new_gain: Resources) -> Self {
        match &self.status {
            GameStatus::Start(_) | GameStatus::InProgress(_) => Game {
                resource_gain: new_gain,
                ..self.clone()
            },
            GameStatus::Finished(_) => Game { ..self.clone() },
        }
    }

    /// Manually pay resources
    pub fn pay_resources(&self, to_pay: &Resources) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                let new_board = subtract_resources(board.clone(), to_pay);

                let (b, res) = match new_board {
                    Ok(b) => (b, GameActionResult::Success),
                    Err(e) => handle_action_error(board, &self.deck, e),
                };
                Game {
                    status: GameStatus::InProgress(calculate_board(b, &self.deck)),
                    action_status: res,
                    ..self.clone()
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => Game {
                action_status: GameActionResult::NothingPayed,
                ..self.clone()
            },
        }
    }

    /// Increases Reputation by given value, maxes out at MAX_VALUE (see Reputation implementation)
    pub fn increase_reputation(&self, value: &Reputation) -> Self {
        match &self.status {
            GameStatus::InProgress(b) => {
                let new_board = add_reputation(b.clone(), value);
                Game {
                    status: GameStatus::InProgress(calculate_board(new_board, &self.deck)),
                    action_status: GameActionResult::Success,
                    ..self.clone()
                }
            }
            GameStatus::Start(b) => {
                let new_board = add_reputation(b.clone(), value);
                Game {
                    status: GameStatus::Start(calculate_board(new_board, &self.deck)),
                    action_status: GameActionResult::Success,
                    ..self.clone()
                }
            }
            GameStatus::Finished(_) => Game {
                action_status: InvalidAction,
                ..self.clone()
            },
        }
    }

    /// Decreases Reputation by given value, bottoms out at 0
    pub fn decrease_reputation(&self, value: &Reputation) -> Self {
        match &self.status {
            GameStatus::InProgress(b) => {
                let new_board = subtract_reputation(b.clone(), value);
                Game {
                    status: GameStatus::InProgress(calculate_board(new_board, &self.deck)),
                    action_status: GameActionResult::Success,
                    ..self.clone()
                }
            }
            GameStatus::Start(b) => {
                let new_board = subtract_reputation(b.clone(), value);
                Game {
                    status: GameStatus::Start(calculate_board(new_board, &self.deck)),
                    action_status: GameActionResult::Success,
                    ..self.clone()
                }
            }
            GameStatus::Finished(_) => Game {
                action_status: InvalidAction,
                ..self.clone()
            },
        }
    }


    /// Try anc closes an Oopsie card. Will roll a dice to calculate the costs.
    fn handle_non_oopsie_close(&self, result: ActionResult<Board>) -> Self {
        match result {
            Ok(b) => Game {
                status: GameStatus::InProgress(calculate_board(b, &self.deck)),
                action_status: GameActionResult::Success,
                ..self.clone()
            },
            Err(err) => {
                let (b, r) = handle_action_error(self.get_board(), &self.deck, err);
                Game {
                    status: GameStatus::InProgress(calculate_board(b, &self.deck)),
                    action_status: r,
                    ..self.clone()
                }
            }
        }
    }

    /// Close a card if allowed.
    pub fn close_card(&self, card_id: &Uuid) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                if let Some(card_to_close) = board.open_cards.get(card_id) {
                    match &**card_to_close {
                        Card::Evaluation(_) => self
                            .handle_non_oopsie_close(close_evaluation_card(board.clone(), card_id)),
                        Card::Attack(_) => self.handle_non_oopsie_close(
                            manually_close_attack_card(board.clone(), card_id),
                        ),
                        Card::Event(_) => {
                            self.handle_non_oopsie_close(close_event_card(board.clone(), card_id))
                        }
                        Card::Lucky(_) => {
                            self.handle_non_oopsie_close(close_lucky_card(board.clone(), card_id))
                        }
                        Card::Oopsie(_) => {
                            let result = try_and_pay_for_oopsie_fix(
                                board.clone(),
                                card_id,
                                self.fix_multiplier.clone(),
                            );
                            match result {
                                Ok((b, r)) => Game {
                                    status: GameStatus::InProgress(b),
                                    action_status: OopsieFixed(r),
                                    ..self.clone()
                                },
                                Err(e) => match e {
                                    ActionError::NotEnoughResources(b, r) => Game {
                                        status: GameStatus::InProgress(b),
                                        action_status: FixFailed(r),
                                        ..self.clone()
                                    },
                                    _ => Game {
                                        action_status: InvalidAction,
                                        ..self.clone()
                                    },
                                },
                            }
                        }
                    }
                } else {
                    Game {
                        action_status: InvalidAction,
                        ..self.clone()
                    }
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => self.clone(),
        }
    }

    /// Method to check if a card is activated. The card itself is not modified, therefore there is no
    /// way to get this information directly form the card. We need to ask the board.
    pub fn is_card_activated(&self, card_id: &Uuid) -> bool {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) | GameStatus::Finished(b) => {
                b.cards_to_use.contains(card_id)
            }
        }
    }

    /// Method to get the cards played and cards total.
    pub fn get_card_count(&self) -> CardCount {
        CardCount {
            played_cards: *&self.deck.played_cards,
            total_cards: *&self.deck.total,
        }
    }
}

fn handle_action_error(board: &Board, deck: &Deck, err: ActionError) -> (Board, GameActionResult) {
    match err {
        ActionError::AttackForceClosed(b) => (b.clone(), GameActionResult::AttackForceClosed),
        ActionError::NoCardsLeft => (board.clone(), InvalidAction),
        ActionError::WrongCardType(b) | ActionError::InvalidState(b) => {
            (calculate_board(b, deck), InvalidAction)
        }
        ActionError::NotEnoughResources(_, _) => {
            (board.clone(), GameActionResult::NotEnoughResources)
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;
    use std::collections::{HashMap, HashSet};

    use crate::cards::properties::cost_modifier::tests::FakeCostModifier;
    use crate::cards::properties::cost_modifier::CostModifier;
    use crate::cards::properties::effect::Effect;
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
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
    use crate::world::board::Board;
    use crate::world::deck::Deck;
    use crate::world::game::{Game, GameActionResult, GameStatus};
    use crate::world::reputation::Reputation;
    use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
    use crate::world::resources::Resources;

    #[derive(Clone)]
    struct TestDeck {
        cards: Vec<Card>,
        start_deck: Deck,
    }

    impl TestDeck {
        fn init_test_deck() -> Self {
            let first_card = Card::from(LuckyCard {
                effect: Effect::OnUsingForFix(
                    FakeEffectDescription.fake(),
                    CostModifier::Decrease(Resources::new(20)),
                ),
                ..FakeLuckyCard.fake()
            });
            let second_card = Card::from(EventCard {
                effect: Effect::OnNextFix(
                    FakeEffectDescription.fake(),
                    CostModifier::Increase(Resources::new(10)),
                ),
                ..FakeEventCard.fake()
            });
            let third_card = Card::from(OopsieCard {
                fix_cost: FixCost::new(10, 20).unwrap(),
                ..FakeOopsieCard.fake()
            });
            let fourth_card = Card::from(EventCard {
                effect: Effect::NOP,
                ..FakeEventCard.fake()
            });
            let fifth_card = Card::from(FakeOopsieCard.fake::<OopsieCard>());
            let sixth_card = Card::from(FakeAttackCard.fake::<AttackCard>());
            let seventh_card = Card::from(EventCard {
                effect: Effect::OnNextFix(FakeEffectDescription.fake(), FakeCostModifier.fake()),
                ..FakeEventCard.fake()
            });
            let cards = vec![
                first_card.clone(),
                second_card.clone(),
                third_card.clone(),
                fourth_card.clone(),
                fifth_card.clone(),
                sixth_card.clone(),
                seventh_card.clone(),
            ];
            let start_deck = Deck {
                remaining_cards: cards.clone(),
                played_cards: 0,
                total: cards.len(),
            };
            TestDeck {
                cards,
                start_deck,
            }
        }
    }

    #[test]
    fn create_creates_board_from_deck_and_sets_parameters() {
        let test_deck = TestDeck::init_test_deck();
        let expectation = Game {
            deck: test_deck.start_deck.clone(),
            status: GameStatus::Start(Board {
                current_resources: Resources::new(0),
                current_reputation: Reputation::new(50),
                drawn_card: None,
                open_cards: HashMap::new(),
                cards_to_use: HashSet::new(),
                cost_modifier: None,
                turns_remaining: test_deck.start_deck.total,
            }),
            action_status: GameActionResult::Success,
            resource_gain: Resources::new(10),
            fix_multiplier: ResourceFixMultiplier::new(2),
        };

        let sut = Game::create(
            test_deck.start_deck,
            Resources::new(10),
            ResourceFixMultiplier::new(2),
        );

        assert_eq!(sut, expectation);
    }

    #[test]
    fn draws_two_cards_calculates_fix_modifier_from_effect() {
        let test_deck = TestDeck::init_test_deck();
        let resource_gain = Resources::new(10);
        let sut = Game::create(
            test_deck.start_deck.clone(),
            resource_gain.clone(),
            ResourceFixMultiplier::new(2),
        );

        let game_after_round_1 = sut.next_round();
        assert_eq!(game_after_round_1.action_status, GameActionResult::Success);
        let board_after_round_1 = get_board_from_in_progress(&game_after_round_1);
        assert!(board_after_round_1.drawn_card.is_some());
        assert_eq!(board_after_round_1.open_cards.len(), 1);
        assert_eq!(board_after_round_1.cost_modifier, None);
        assert_eq!(
            board_after_round_1.turns_remaining,
            test_deck.cards.len() - 1
        );
        assert_eq!(board_after_round_1.current_resources, resource_gain.clone());
        assert!(board_after_round_1.cards_to_use.is_empty());

        let game_after_round_2 = game_after_round_1.next_round();
        assert_eq!(game_after_round_2.action_status, GameActionResult::Success);
        let board_after_round_2 = get_board_from_in_progress(&game_after_round_2);
        assert!(board_after_round_2.drawn_card.is_some());
        assert_eq!(board_after_round_2.open_cards.len(), 2);
        assert_eq!(
            board_after_round_2.cost_modifier,
            Some(CostModifier::Increase(Resources::new(10)))
        );
        assert_eq!(
            board_after_round_2.turns_remaining,
            test_deck.cards.len() - 2
        );
        assert_eq!(board_after_round_2.current_resources, &resource_gain * 2);
        assert!(board_after_round_2.cards_to_use.is_empty());
    }

    #[test]
    fn draws_one_card_and_activates_lucky_card() {
        let test_deck = TestDeck::init_test_deck();
        let resource_gain = Resources::new(10);
        let sut = Game::create(
            test_deck.start_deck.clone(),
            resource_gain.clone(),
            ResourceFixMultiplier::new(2),
        );

        let game_after_round_1 = sut.next_round();
        assert_eq!(game_after_round_1.action_status, GameActionResult::Success);
        let board_after_round_1 = get_board_from_in_progress(&game_after_round_1);
        assert!(board_after_round_1.drawn_card.is_some());
        assert_eq!(board_after_round_1.open_cards.len(), 1);
        assert_eq!(board_after_round_1.cost_modifier, None);
        assert_eq!(
            board_after_round_1.turns_remaining,
            test_deck.cards.len() - 1
        );
        assert_eq!(board_after_round_1.current_resources, resource_gain.clone());
        assert!(board_after_round_1.cards_to_use.is_empty());

        let card_id = &board_after_round_1.drawn_card.clone().unwrap().id;

        let game_after_activate = game_after_round_1.activate_lucky_card(card_id);

        assert_eq!(game_after_activate.action_status, GameActionResult::Success);
        let board_after_activate = get_board_from_in_progress(&game_after_activate);
        assert_eq!(
            board_after_activate.drawn_card,
            board_after_round_1.drawn_card
        );
        assert_eq!(
            board_after_activate.open_cards,
            board_after_round_1.open_cards
        );
        assert_eq!(
            board_after_activate.cost_modifier,
            Some(CostModifier::Decrease(Resources::new(20)))
        );
        assert_eq!(
            board_after_activate.turns_remaining,
            board_after_round_1.turns_remaining
        );
        assert_eq!(
            board_after_activate.current_resources,
            board_after_round_1.current_resources
        );
        assert!(board_after_activate.cards_to_use.contains(card_id));
        assert!(game_after_activate.is_card_activated(card_id))
    }

    #[test]
    fn draws_one_card_and_activates_and_deactivates_lucky_card() {
        let test_deck = TestDeck::init_test_deck();
        let resource_gain = Resources::new(10);
        let sut = Game::create(
            test_deck.start_deck.clone(),
            resource_gain.clone(),
            ResourceFixMultiplier::new(2),
        );

        let game_after_round_1 = sut.next_round();
        assert_eq!(game_after_round_1.action_status, GameActionResult::Success);
        let board_after_round_1 = get_board_from_in_progress(&game_after_round_1);
        assert!(board_after_round_1.drawn_card.is_some());
        assert_eq!(board_after_round_1.open_cards.len(), 1);
        assert_eq!(board_after_round_1.cost_modifier, None);
        assert_eq!(
            board_after_round_1.turns_remaining,
            test_deck.cards.len() - 1
        );
        assert_eq!(board_after_round_1.current_resources, resource_gain.clone());
        assert!(board_after_round_1.cards_to_use.is_empty());

        let card_id = &board_after_round_1.drawn_card.clone().unwrap().id;

        let game_after_activate = game_after_round_1.activate_lucky_card(card_id);

        let game_after_deactivate = game_after_activate.deactivate_lucky_card(card_id);

        assert_eq!(
            game_after_deactivate.action_status,
            GameActionResult::Success
        );
        let board_after_deactivate = get_board_from_in_progress(&game_after_deactivate);
        assert_eq!(
            board_after_deactivate.drawn_card,
            board_after_round_1.drawn_card
        );
        assert_eq!(
            board_after_deactivate.open_cards,
            board_after_round_1.open_cards
        );
        assert_eq!(board_after_deactivate.cost_modifier, None);
        assert_eq!(
            board_after_deactivate.turns_remaining,
            board_after_round_1.turns_remaining
        );
        assert_eq!(
            board_after_deactivate.current_resources,
            board_after_round_1.current_resources
        );
        assert!(board_after_deactivate.cards_to_use.is_empty());
        assert!(!game_after_deactivate.is_card_activated(card_id))
    }

    fn get_board_from_in_progress(game: &Game) -> Board {
        match &game.status {
            GameStatus::InProgress(b) => b.clone(),
            GameStatus::Start(_) | GameStatus::Finished(_) => panic!("Must be InProgress"),
        }
    }
}
