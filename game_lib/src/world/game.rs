use std::collections::HashMap;
use uuid::Uuid;

use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::ActionError;
use crate::world::actions::add_resources::add_resources;
use crate::world::actions::calculate_board::calculate_board;
use crate::world::actions::close_attack::manually_close_attack_card;
use crate::world::actions::close_oopsie::try_and_pay_for_oopsie_fix;
use crate::world::actions::draw_card::draw_card_and_place_on_board;
use crate::world::actions::remove_resources::remove_resources;
use crate::world::actions::use_lucky_card::{activate_lucky_card, deactivate_lucky_card};
use crate::world::board::Board;
use crate::world::deck::{CardRc, Deck};
use crate::world::game::GameActionResult::InvalidAction;
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Debug, Clone)]
pub enum GameStatus {
    Start(Board),
    InProgress(Board),
    Finished(Board),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Game {
    deck: Deck,
    pub status: GameStatus,
    pub action_status: GameActionResult,
    pub resource_gain: Resources,
    pub fix_multiplier: ResourceFixMultiplier,
}

pub struct CardCount {
    pub played_cards: usize,
    pub total_cards: usize,
}

impl Game {
    pub fn get_open_cards(&self) -> HashMap<Uuid, CardRc> {
        self.get_board().open_cards.clone()
    }

    pub fn set_fix_multiplier(&self, resource_fix_multiplier: ResourceFixMultiplier) -> Game {
        Game {
            fix_multiplier: resource_fix_multiplier,
            ..self.clone()
        }
    }

    pub fn activate_lucky_card(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match activate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::Start(calculate_board(new_board, &self.deck)),
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

    pub fn deactivate_lucky_card(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match deactivate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::Start(calculate_board(new_board, &self.deck)),
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

    pub fn get_current_fix_modifier(&self) -> Option<FixModifier> {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) | GameStatus::Finished(b) => {
                b.fix_modifier.clone()
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

    pub fn create(
        deck: Deck,
        initial_resource_gain: Resources,
        fix_multiplier: ResourceFixMultiplier,
    ) -> Self {
        let board = Board::init(&deck, Resources::new(0));
        let status = GameStatus::Start(calculate_board(board, &deck));

        Game {
            deck,
            status,
            action_status: GameActionResult::Success,
            resource_gain: initial_resource_gain.clone(),
            fix_multiplier,
        }
    }

    pub fn next_round(&self) -> Self {
        if let Ok((new_deck, board)) =
            draw_card_and_place_on_board(self.deck.clone(), self.get_board().clone())
        {
            let board_with_added_resources = add_resources(board, &self.resource_gain);
            let new_board = calculate_board(board_with_added_resources, &new_deck);

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

    pub fn set_resource_gain(&self, new_gain: Resources) -> Self {
        match &self.status {
            GameStatus::Start(_) | GameStatus::InProgress(_) => Game {
                resource_gain: new_gain,
                ..self.clone()
            },
            GameStatus::Finished(_) => Game { ..self.clone() },
        }
    }

    pub fn pay_resources(&self, to_pay: &Resources) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                let new_board = remove_resources(board.clone(), to_pay);

                let (b, res) = match new_board {
                    Ok(b) => (b, GameActionResult::Success),
                    Err(e) => handle_action_error(board, &self.deck,e),
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

    pub fn close_card(&self, card_id: &Uuid) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                let result = if let Some(card_to_close) = board.open_cards.get(card_id) {
                    match &**card_to_close {
                        Card::Attack(_) => manually_close_attack_card(board.clone(), card_id),
                        Card::Oopsie(_) => try_and_pay_for_oopsie_fix(
                            board.clone(),
                            card_id,
                            self.fix_multiplier.clone(),
                        ),
                        Card::Event(_) | Card::Lucky(_) => {
                            Err(ActionError::WrongCardType(board.clone()))
                        }
                    }
                } else {
                    Err(ActionError::InvalidState(board.clone()))
                };
                match result {
                    Ok(b) => Game {
                        status: GameStatus::InProgress(calculate_board(b, &self.deck)),
                        action_status: GameActionResult::Success,
                        ..self.clone()
                    },
                    Err(err) => {
                        let (b, r) = handle_action_error(board, &self.deck, err);
                        Game {
                            status: GameStatus::InProgress(calculate_board(b, &self.deck)),
                            action_status: r,
                            ..self.clone()
                        }
                    }
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => self.clone(),
        }
    }

    pub fn is_card_activated(&self, card_id: &Uuid) -> bool {
        match &self.status {
            GameStatus::Start(b)
            | GameStatus::InProgress(b)
            | GameStatus::Finished(b) => b.cards_to_use.contains(card_id)
        }
    }

    pub fn get_card_count(&self) -> CardCount {
        CardCount {
            played_cards: *&self.deck.played_cards,
            total_cards: *&self.deck.total,
        }
    }
}

fn handle_action_error(board: &Board, deck: &Deck, err: ActionError) -> (Board, GameActionResult) {
    match err {
        ActionError::NoCardsLeft => (board.clone(), InvalidAction),
        ActionError::WrongCardType(b)
        | ActionError::AttackForceClosed(b)
        | ActionError::InvalidState(b) => (calculate_board(b, deck), InvalidAction),
        ActionError::NotEnoughResources(_, _) => (board.clone(), GameActionResult::NothingPayed),
    }
}
