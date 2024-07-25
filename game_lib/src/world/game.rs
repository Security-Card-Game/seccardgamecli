use std::collections::HashMap;

use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::cards::properties::duration::Duration;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::{Card, CardTrait};
use crate::cards::types::oopsie::OopsieCard;
use crate::world::actions::close_attack::manually_close_attack_card;
use crate::world::actions::close_oopsie::try_and_pay_for_oopsie_fix;
use crate::world::actions::draw_card::draw_card_and_place_on_board;
use crate::world::actions::use_lucky_card::{activate_lucky_card, deactivate_lucky_card};
use crate::world::board::{Board, CurrentBoard};
use crate::world::deck::{CardRc, Deck};
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Debug, Clone)]
pub enum GameStatus {
    Start(Board),
    InProgress(Board),
    Finished(Board),
}

#[derive(Debug, Clone)]
pub enum ActionResult {
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
    pub action_status: Option<ActionResult>,
    pub resource_gain: Resources,
    pub fix_multiplier: ResourceFixMultiplier,
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

    fn set_result(&self, action_result: Option<ActionResult>) -> Game {
        Game {
            action_status: action_result,
            ..self.clone()
        }
    }

    pub fn use_card_on_next_fix(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match activate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::Start(new_board),
                        action_status: Some(ActionResult::Success),
                        ..self.clone()
                    },
                    Err(_) => self.set_result(Some(ActionResult::InvalidAction)),
                }
            }
            GameStatus::Finished(_) => self.set_result(Some(ActionResult::InvalidAction)),
        }
    }

    pub fn set_fix_multiplier(&self, resource_fix_multiplier: ResourceFixMultiplier) -> Game {
        Game {
            fix_multiplier: resource_fix_multiplier,
            ..self.clone()
        }
    }

    pub fn activate_card(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match activate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::Start(new_board),
                        action_status: Some(ActionResult::Success),
                        ..self.clone()
                    },
                    Err(_) => self.set_result(Some(ActionResult::InvalidAction)),
                }
            }
            GameStatus::Finished(_) => self.set_result(Some(ActionResult::InvalidAction)),
        }
    }

    pub fn deactivate_card(&self, card_id: &Uuid) -> Game {
        match &self.status {
            GameStatus::Start(b) | GameStatus::InProgress(b) => {
                match deactivate_lucky_card(b.clone(), card_id) {
                    Ok(new_board) => Game {
                        status: GameStatus::Start(new_board),
                        action_status: Some(ActionResult::Success),
                        ..self.clone()
                    },
                    Err(_) => self.set_result(Some(ActionResult::InvalidAction)),
                }
            }
            GameStatus::Finished(_) => self.set_result(Some(ActionResult::InvalidAction)),
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
        let status = GameStatus::Start(board);

        Game {
            deck,
            status,
            action_status: None,
            resource_gain: initial_resource_gain.clone(),
            fix_multiplier,
        }
    }

    pub fn next_round(&self) -> Self {
        if let Ok((new_deck, board)) =
            draw_card_and_place_on_board(self.deck.clone(), self.get_board().clone())
        {
            // todo: move to function/action
            let new_board = Board {
                current_resources: board.current_resources + self.resource_gain.clone(),
                ..board
            };

            let status = if (new_board.turns_remaining == 0) {
                GameStatus::Finished(new_board)
            } else {
                GameStatus::InProgress(new_board)
            };
            Game {
                action_status: Some(ActionResult::Success),
                deck: new_deck,
                status,
                ..self.clone()
            }
        } else {
            Game {
                action_status: Some(ActionResult::InvalidAction),
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

    pub fn pay_resources(&self, to_pay: &Resources) -> Payment {
        match &self.status {
            GameStatus::InProgress(board) => {
                if to_pay > &board.current_resources {
                    Payment::NotEnoughResources(self.clone())
                } else {
                    // todo: move to function/action
                    let new_board = Board {
                        current_resources: &board.current_resources - to_pay,
                        ..board.clone()
                    };
                    let game = Game {
                        status: GameStatus::InProgress(new_board),
                        action_status: Some(ActionResult::Success),
                        ..self.clone()
                    };
                    Payment::Payed(game)
                }
            }
            GameStatus::Start(_) | GameStatus::Finished(_) => Payment::NothingPayed(self.clone()),
        }
    }

    pub fn close_card(&self, card_id: &Uuid) -> Self {
        let board = match &self.status {
            GameStatus::InProgress(board) => {

                if let Some(card_to_close) = board.open_cards.get(card_id) {
                    match &**card_to_close {
                        Card::Attack(_) => manually_close_attack_card(board.clone(), card_id),
                        Card::Oopsie(_) => try_and_pay_for_oopsie_fix(board.clone(), card_id),
                        Card::Event(_)
                        | Card::Lucky(_) => // todo: remove card from open cards
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

        let mut new_active_cards = self.active_cards.clone();
        new_active_cards.remove(card_id);

        Game {
            status: GameStatus::InProgress(new_board),
            action_status: self.action_status.clone(),
            resource_gain: self.resource_gain.clone(),
            resource_effects: new_resource_effects,
            active_cards: new_active_cards,
            fix_multiplier: self.fix_multiplier.clone(),
        }
    }

    fn close_attack_card(&self, board: &CurrentBoard, card_id: &Uuid, ac: &AttackCard) -> Self {
        match ac.duration {
            Duration::Rounds(_) | Duration::UntilClosed => {
                let game = self.do_close_card(board, card_id);

                let mut new_active_cards = self.active_cards.clone();
                new_active_cards.remove(card_id);

                Game {
                    status: game.status.clone(),
                    action_status: Some(ActionResult::AttackForceClosed),
                    resource_gain: game.resource_gain.clone(),
                    resource_effects: game.resource_effects.clone(),
                    active_cards: new_active_cards,
                    fix_multiplier: self.fix_multiplier.clone(),
                }
            }
            Duration::None => self.do_close_card(board, card_id),
        }
    }

    fn close_oopsie_card(&self, card_id: &Uuid, oc: &OopsieCard) -> Self {
        let fix_cost = roll_dice_for_card(oc, &self.fix_multiplier);
        let actual_cost = if let Some(modifier) = self.get_current_fix_modifier() {
            match modifier {
                FixModifier::Increase(r) => fix_cost + r,
                FixModifier::Decrease(r) => fix_cost - r,
            }
        } else {
            fix_cost
        };

        let payed = self.pay_resources(&actual_cost);
        let game = match payed {
            Payment::Payed(g) => {
                let game = g.set_action_result(ActionResult::OopsieFixed(actual_cost));
                game.do_close_card(game.get_board(), card_id)
            }
            Payment::NotEnoughResources(g) => {
                let failed_fix = g.set_action_result(ActionResult::FixFailed(actual_cost));
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
            active_cards: self.active_cards.clone(),
            fix_multiplier: self.fix_multiplier.clone(),
        }
    }
}

fn roll_dice_for_card(card: &OopsieCard, multiplier: &ResourceFixMultiplier) -> Resources {
    let mut rng = thread_rng();
    let cost = rng.gen_range(card.fix_cost.min.value().clone()..card.fix_cost.max.value().clone());
    Resources::new(cost) * multiplier
}
