use crate::{Message, SecCardGameApp};
use game_lib::world::game::GameActionResult;
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) enum Command {
    SetResourceGain(usize),
    PayResources(usize),
    SetMultiplier(isize),
    CloseCard(Uuid),
    DeactivateCard(Uuid),
    ActivateCard(Uuid),
}

pub(crate) trait CommandHandler {
    fn process_command(&mut self);
}

impl CommandHandler for SecCardGameApp {
    fn process_command(&mut self) {
        let cmd = {
            let cmd_borrow = self.command.borrow();
            cmd_borrow.clone()
        };

        if let Some(cmd) = cmd {
            self.handle_command(cmd);
        }
    }
}

impl SecCardGameApp {
    fn handle_command(&mut self, msg: Command) {
        self.input.message = Message::None;

        match msg {
            Command::SetResourceGain(res) => self.handle_set_resource_gain(res),
            Command::PayResources(res) => self.handle_pay_resources(res),
            Command::SetMultiplier(m) => self.handle_set_multiplier(m),
            Command::CloseCard(card_id) => self.handle_card_closed(card_id),
            Command::DeactivateCard(card_id) => self.handle_deactivate_card(card_id),
            Command::ActivateCard(card_id) => self.handle_activate_card(card_id)
        }

        self.reset_command(); // removes the command from the state after it is executed
        self.process_game_action_status(); // takes care of displaying results of actions in the UI
    }

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
            GameActionResult::NotEnoughResources => {
                self.input.message = Message::Warning("Not enough resources!".to_string());
            }
            GameActionResult::NothingPayed => {
                self.input.pay_res = "0".to_string();
                self.input.message = Message::None;
            }
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