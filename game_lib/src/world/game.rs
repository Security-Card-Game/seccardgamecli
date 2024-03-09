use std::collections::HashMap;

use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::cards::properties::effect::Effect;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::card_model::Card;
use crate::world::board::CurrentBoard;
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
    resource_effects: HashMap<Uuid, FixModifier>,
}

impl Game {
    pub fn get_open_cards(&self) -> HashMap<Uuid, CardRc> {
        self.get_board().open_cards.clone()
    }

    pub fn get_current_fix_modifier(&self) -> Option<FixModifier> {
        if (self.resource_effects.is_empty()) {
            None
        } else {
            let mut increase = 0;
            let mut decrease = 0;
            for (_, modifier) in self.resource_effects.iter() {
                match modifier {
                    FixModifier::Increase(r) => {
                        increase += r.value().clone();
                    }
                    FixModifier::Decrease(r) => {
                        decrease += r.value().clone();
                    }
                }
            }
            let value = increase as isize - decrease as isize;
            if (value <= 0) {
                Some(FixModifier::Decrease(Resources::new(value.abs() as usize)))
            } else {
                Some(FixModifier::Increase(Resources::new(value as usize)))
            }
        }
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
            resource_effects: HashMap::new(),
        }
    }

    pub fn next_round(&self) -> Self {
        let mut drawn_card = None;
        let status = match &self.status {
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                let new_board = board.next_round(self.resource_gain.clone());
                drawn_card = new_board.drawn_card.clone();
                if new_board.turns_remaining == 0 {
                    GameStatus::Finished(new_board)
                } else {
                    GameStatus::InProgress(new_board)
                }
            }
            GameStatus::Finished(board) => GameStatus::Finished(board.clone()),
        };

        let resource_effects = match drawn_card {
            None => self.resource_effects.clone(),
            Some(card) => match &*card.card {
                Card::Event(c) => match &c.effect {
                    Effect::OnNextFix(_, m) => {
                        let mut new_resource_effect = self.resource_effects.clone();
                        new_resource_effect.insert(card.id.clone(), m.clone());
                        new_resource_effect
                    }
                    _ => self.resource_effects.clone(),
                },
                Card::Attack(_) => self.resource_effects.clone(),
                Card::Oopsie(_) => self.resource_effects.clone(),
                Card::Lucky(_) => self.resource_effects.clone(),
            },
        };

        Game {
            status,
            resource_gain: self.resource_gain.clone(),
            resource_effects,
        }
    }

    pub fn set_resource_gain(&self, new_gain: Resources) -> Self {
        match &self.status {
            GameStatus::Start(_) | GameStatus::InProgress(_) => Game {
                status: self.status.clone(),
                resource_gain: new_gain,
                resource_effects: self.resource_effects.clone(),
            },
            GameStatus::Finished(board) => Game {
                status: self.status.clone(),
                resource_gain: self.resource_gain.clone(),
                resource_effects: self.resource_effects.clone(),
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
                    resource_effects: self.resource_effects.clone(),
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
                let mut new_resource_effects = self.resource_effects.clone();
                new_resource_effects.remove(card_id);
                Game {
                    status: GameStatus::InProgress(new_board),
                    resource_gain: self.resource_gain.clone(),
                    resource_effects: new_resource_effects,
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => self.clone(),
        }
    }
}
