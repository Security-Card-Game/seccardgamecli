use std::collections::HashMap;

use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::cards::properties::duration::Duration;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::{Card, CardTrait};
use crate::cards::types::oopsie::OopsieCard;
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
pub enum ActionResult {
    OopsieFixed,
    FixFailed,
    AttackForceClosed,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub status: GameStatus,
    pub action_status: Option<ActionResult>,
    pub resource_gain: Resources,
    resource_effects: HashMap<Uuid, FixModifier>,
}

pub enum Payment {
    Payed(Game),
    NotEnoughResources(Game),
    NothingPayed(Game),
}

impl Game {
    pub fn get_open_cards(&self) -> HashMap<Uuid, CardRc> {
        self.get_board().open_cards.clone()
    }

    pub fn use_card_on_next_fix(&self, card_id: &Uuid) -> Game {
        let open_cards = self.get_open_cards();
        if let Some(card) = open_cards.get(card_id) {
            let modifier = match card.effect() {
                Effect::OnUsingForFix(_, m) => Some(m),
                _ => None,
            };
            let res_effects = match modifier {
                Some(m) => {
                    let mut new_resource_effects = self.resource_effects.clone();
                    new_resource_effects.insert(card_id.clone(), m.clone());
                    new_resource_effects
                }
                None => self.resource_effects.clone(),
            };
            self.set_resource_effects(res_effects)
        } else {
            self.clone()
        }
    }

    pub fn do_not_use_card_on_next_fix(&self, card_id: &Uuid) -> Game {
        let open_cards = self.get_open_cards();
        if let Some(card) = open_cards.get(card_id) {
            let mut new_resource_effects = self.resource_effects.clone();
            new_resource_effects.remove(card_id);
            self.set_resource_effects(new_resource_effects)
        } else {
            self.clone()
        }
    }

    fn set_resource_effects(&self, resource_effects: HashMap<Uuid, FixModifier>) -> Game {
        Game {
            status: self.status.clone(),
            resource_effects: resource_effects.clone(),
            resource_gain: self.resource_gain.clone(),
            action_status: self.action_status.clone(),
        }
    }

    pub fn get_current_fix_modifier(&self) -> Option<FixModifier> {
        if self.resource_effects.is_empty() {
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
            action_status: None,
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
            action_status: None,
            resource_gain: self.resource_gain.clone(),
            resource_effects,
        }
    }

    pub fn set_resource_gain(&self, new_gain: Resources) -> Self {
        match &self.status {
            GameStatus::Start(_) | GameStatus::InProgress(_) => Game {
                status: self.status.clone(),
                action_status: self.action_status.clone(),
                resource_gain: new_gain,
                resource_effects: self.resource_effects.clone(),
            },
            GameStatus::Finished(board) => Game {
                status: self.status.clone(),
                action_status: self.action_status.clone(),
                resource_gain: self.resource_gain.clone(),
                resource_effects: self.resource_effects.clone(),
            },
        }
    }

    pub fn pay_resources(&self, to_pay: Resources) -> Payment {
        match &self.status {
            GameStatus::InProgress(board) => {
                if to_pay > board.current_resources {
                    Payment::NotEnoughResources(self.clone())
                } else {
                    let new_board = board.pay_resources(&to_pay);
                    let game = Game {
                        status: GameStatus::InProgress(new_board),
                        action_status: self.action_status.clone(),
                        resource_gain: self.resource_gain.clone(),
                        resource_effects: self.resource_effects.clone(),
                    };
                    Payment::Payed(game)
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => Payment::NothingPayed(self.clone()),
        }
    }

    fn set_available_resources(&self, available_resources: Resources) -> Game {
        let board = match &self.status {
            GameStatus::Start(board) | GameStatus::InProgress(board) => CurrentBoard {
                current_resources: available_resources,
                turns_remaining: board.turns_remaining.clone(),
                deck: board.deck.clone(),
                open_cards: board.open_cards.clone().clone(),
                drawn_card: board.drawn_card.clone(),
            },
            GameStatus::Finished(board) => board.clone(),
        };
        self.set_board(board)
    }

    fn set_board(&self, board: CurrentBoard) -> Game {
        let new_status = match &self.status {
            GameStatus::Start(_) => GameStatus::Start(board),
            GameStatus::InProgress(_) => GameStatus::InProgress(board),
            GameStatus::Finished(_) => GameStatus::Finished(board),
        };
        Game {
            status: new_status,
            action_status: self.action_status.clone(),
            resource_gain: self.resource_gain.clone(),
            resource_effects: self.resource_effects.clone(),
        }
    }

    pub fn close_card(&self, card_id: &Uuid) -> Self {
        match &self.status {
            GameStatus::InProgress(board) => {
                if let Some(card_to_close) = board.open_cards.get(card_id) {
                    match &**card_to_close {
                        Card::Event(ec) => self.do_close_card(board, card_id),
                        Card::Attack(ac) => self.close_attack_card(board, card_id, ac),
                        Card::Oopsie(oc) => self.close_oopsie_card(board, card_id, oc),
                        Card::Lucky(_) => self.do_close_card(board, card_id),
                    }
                } else {
                    self.clone()
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => self.clone(),
        }
    }

    fn do_close_card(&self, board: &CurrentBoard, card_id: &Uuid) -> Self {
        let new_board = board.close_card(card_id);
        let mut new_resource_effects = self.resource_effects.clone();
        new_resource_effects.remove(card_id);
        Game {
            status: GameStatus::InProgress(new_board),
            action_status: self.action_status.clone(),
            resource_gain: self.resource_gain.clone(),
            resource_effects: new_resource_effects,
        }
    }

    fn close_attack_card(&self, board: &CurrentBoard, card_id: &Uuid, ac: &AttackCard) -> Self {
        match ac.duration {
            Duration::Rounds(_) | Duration::UntilClosed => {
                let game = self.do_close_card(board, card_id);
                Game {
                    status: game.status.clone(),
                    action_status: Some(ActionResult::AttackForceClosed),
                    resource_gain: game.resource_gain.clone(),
                    resource_effects: game.resource_effects.clone(),
                }
            }
            Duration::None => self.do_close_card(board, card_id),
        }
    }

    fn close_oopsie_card(&self, board: &CurrentBoard, card_id: &Uuid, oc: &OopsieCard) -> Self {
        let fix_cost = roll_dice_for_card(oc);
        let actual_cost = if let Some(modifier) = self.get_current_fix_modifier() {
            match modifier {
                FixModifier::Increase(r) => fix_cost + r,
                FixModifier::Decrease(r) => fix_cost - r,
            }
        } else {
            fix_cost
        };

        let payed = self.pay_resources(actual_cost);
        let game = match payed {
            Payment::Payed(g) => {
                let game = g.set_action_result(ActionResult::OopsieFixed);
                game.do_close_card(game.get_board(), card_id)
            }
            Payment::NotEnoughResources(g) => {
                let failed_fix = g.set_action_result(ActionResult::FixFailed);
                failed_fix.set_available_resources(Resources::new(0))
            }
            Payment::NothingPayed(g) => g.clone(),
        };
        game.reset_resource_modifier()
    }

    fn reset_resource_modifier(&self) -> Self {
        let mut game = self.clone();
        for card_id in self.resource_effects.keys() {
            let game_with_closed_card = game.close_card(card_id);
            game = game_with_closed_card;
        }
        game
    }

    fn set_action_result(&self, action_result: ActionResult) -> Self {
        Game {
            status: self.status.clone(),
            action_status: Some(action_result),
            resource_gain: self.resource_gain.clone(),
            resource_effects: self.resource_effects.clone(),
        }
    }
}

fn roll_dice_for_card(card: &OopsieCard) -> Resources {
    let mut rng = thread_rng();
    let cost = rng.gen_range(card.fix_cost.min.value().clone()..card.fix_cost.max.value().clone());
    Resources::new(cost)
}
