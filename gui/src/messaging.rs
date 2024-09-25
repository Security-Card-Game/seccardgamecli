use crate::{Message, SecCardGameApp};
use game_lib::world::game::GameActionResult;
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

pub(crate) enum UpdateMessage {
    SetResourceGain(usize),
    PayResources(usize),
    SetMultiplier(isize),
}

pub(crate) trait MessageHandler {
    fn handle_message(&mut self, msg: UpdateMessage);
}

impl MessageHandler for SecCardGameApp {
    fn handle_message(&mut self, msg: UpdateMessage) {
        self.input.message = Message::None;

        match msg {
            UpdateMessage::SetResourceGain(res) => self.handle_set_resource_gain(res),
            UpdateMessage::PayResources(res) => self.handle_pay_resources(res),
            UpdateMessage::SetMultiplier(m) => self.handle_set_multiplier(m),
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
