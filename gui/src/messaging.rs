use uuid::Uuid;
use crate::{Message, SecCardGameApp};
use game_lib::world::game::GameActionResult;
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

#[derive(Debug, Clone)]
pub(crate) enum UpdateMessage {
    SetResourceGain(usize),
    PayResources(usize),
    SetMultiplier(isize),
    CloseCard(Uuid),
    DeactivateCard(Uuid),
    ActivateCard(Uuid),
}

pub(crate) trait MessageHandler {
    fn handle_message(&mut self, msg: UpdateMessage);
    fn process_command(&mut self);
}

impl MessageHandler for SecCardGameApp {
    fn handle_message(&mut self, msg: UpdateMessage) {
        self.input.message = Message::None;

        match msg {
            UpdateMessage::SetResourceGain(res) => self.handle_set_resource_gain(res),
            UpdateMessage::PayResources(res) => self.handle_pay_resources(res),
            UpdateMessage::SetMultiplier(m) => self.handle_set_multiplier(m),
            UpdateMessage::CloseCard(card_id) => self.handle_card_closed(card_id),
            UpdateMessage::DeactivateCard(card_id) => self.handle_deactivate_card(card_id),
            UpdateMessage::ActivateCard(card_id) => self.handle_activate_card(card_id)
        }

        self.reset_command();
        self.process_game_action_status();
    }

    fn process_command(&mut self) {
        let cmd = {
            let cmd_borrow = self.command.borrow();
            cmd_borrow.clone()
        };

        if let Some(cmd) = cmd {
            self.handle_message(cmd);
        }
    }
}

impl SecCardGameApp {

    fn reset_command(&mut self) {
        let mut cmd = self.command.borrow_mut();
        *cmd = None;
    }

    fn handle_card_closed(&mut self, card_id: Uuid) {
        let new_game_state = self.game.close_card(&card_id);
        self.game = new_game_state;
    }

    fn handle_activate_card(&mut self, card_id: Uuid) {
        let new_game_state = self.game.activate_lucky_card(&card_id);
        self.game = new_game_state;
    }

    fn handle_deactivate_card(&mut self, card_id: Uuid) {
        let new_game_state = self.game.deactivate_lucky_card(&card_id);
        self.game = new_game_state;
    }


    fn process_game_action_status(&mut self) {
        match &self.game.action_status {
            GameActionResult::OopsieFixed(res) => {
                self.input.message =
                    Message::Success(format!("Fixed for {} resources.", res.value()));
            }
            GameActionResult::FixFailed(res) => {
                self.input.message = Message::Failure(format!(
                    "Fix failed! It would have needed {} resources.",
                    res.value()
                ));
            }
            GameActionResult::AttackForceClosed => {
                self.input.message =
                    Message::Warning("Attack forced to be over".to_string());
            }
            GameActionResult::Payed => {}
            GameActionResult::NotEnoughResources => {}
            GameActionResult::NothingPayed => {}
            GameActionResult::InvalidAction => {
                self.input.message =
                    Message::Failure("Invalid Action!".to_string())
            }
            GameActionResult::Success => {
                self.input.message = Message::None
            }
        }
    }

    fn handle_pay_resources(&mut self, res: usize) {
        self.game = self.game.pay_resources(&Resources::new(res));
        match self.game.action_status {
            GameActionResult::Payed => {}
            GameActionResult::NothingPayed => {
                self.input.pay_res = "0".to_string();
                self.input.message = Message::None;
            }
            GameActionResult::NotEnoughResources => {
                self.input.message = Message::Warning("Not enough resources!".to_string());
            }
            _ => {}
        }
    }

    fn handle_set_multiplier(&mut self, m: isize) {
        if m <= 0 {
            self.input.message = Message::Failure("Invalid Action, must be > 0!".to_string());
        }
        self.game = self
            .game
            .set_fix_multiplier(ResourceFixMultiplier::new(m.unsigned_abs()));
    }

    fn handle_set_resource_gain(&mut self, res: usize) {
        self.game = self.game.set_resource_gain(Resources::new(res));
        self.input.next_res = res.to_string();
    }
}