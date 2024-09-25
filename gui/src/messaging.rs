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
            UpdateMessage::CloseCard(id) => { dbg!("Close card called"); }
            UpdateMessage::DeactivateCard(_) => { dbg!{"Deactivate card called"}; }
            UpdateMessage::ActivateCard(_) => { dbg!{"Activate card called"}; }
        }
        let mut cmd = self.command.borrow_mut();
        *cmd = None
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