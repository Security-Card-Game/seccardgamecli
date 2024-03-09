use std::collections::HashMap;

use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::cards::types::card_model::Card;
use crate::world::current_turn::CurrentBoard;
use crate::world::deck::{CardRc, Deck};
use crate::world::resources::Resources;

#[derive(Debug, Clone)]
pub enum GameStatus {
    Start(CurrentBoard),
    InProgress(CurrentBoard),
    Finished(CurrentBoard),
}

#[derive(Debug, Clone)]
pub struct Game {
    pub status: GameStatus,
    pub resource_gain: Resources,
}

impl Game {
    pub fn get_open_cards(&self) -> HashMap<Uuid, CardRc> {
        self.get_board().open_cards.clone()
    }

    fn get_board(&self) -> &CurrentBoard {
        match self.status {
            GameStatus::Start(ref board)
            | GameStatus::InProgress(ref board)
            | GameStatus::Finished(ref board) => board,
        }
    }

    pub fn create(deck: Deck, initial_resource_gain: Resources) -> Self {
        let board = CurrentBoard::init(deck, Resources::new(0));
        let status = GameStatus::Start(board);

        Game {
            status,
            resource_gain: initial_resource_gain.clone(),
        }
    }

    pub fn next_round(&self) -> Self {
        let status = match &self.status {
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                let new_board = board.next_round(self.resource_gain.clone());
                if new_board.turns_remaining == 0 {
                    GameStatus::Finished(new_board)
                } else {
                    GameStatus::InProgress(new_board)
                }
            }
            GameStatus::Finished(board) => GameStatus::Finished(board.clone()),
        };

        Game {
            status,
            resource_gain: self.resource_gain.clone(),
        }
    }

    pub fn set_resource_gain(&self, new_gain: Resources) -> Self {
        match &self.status {
            GameStatus::Start(_) | GameStatus::InProgress(_) => Game {
                status: self.status.clone(),
                resource_gain: new_gain,
            },
            GameStatus::Finished(board) => Game {
                status: self.status.clone(),
                resource_gain: self.resource_gain.clone(),
            },
        }
    }

    pub fn pay_resources(&self, to_pay: Resources) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                let new_board = board.pay_resources(&to_pay);
                Game {
                    status: GameStatus::InProgress(new_board),
                    resource_gain: self.resource_gain.clone(),
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => self.clone(),
        }
    }

    pub fn roll_dice_for_card(card: CardRc) -> usize {
        let mut rng = thread_rng();
        match &*card {
            Card::Event(_) => 0,
            Card::Attack(_) => 0,
            Card::Oopsie(c) => {
                rng.gen_range(c.fix_cost.min.value().clone()..c.fix_cost.max.value().clone())
            }
            Card::Lucky(_) => 0,
        }
    }

    pub fn close_card(&self, card_id: &Uuid) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                let new_board = board.close_card(card_id);
                Game {
                    status: GameStatus::InProgress(new_board),
                    resource_gain: self.resource_gain.clone(),
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => self.clone(),
        }
    }
}
