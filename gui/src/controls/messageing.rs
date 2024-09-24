use game_lib::world::game::GameActionResult;
use game_lib::world::resources::Resources;
use crate::{Message, SecCardGameApp};

pub(crate) enum UpdateMessage {
    SetResourceGain(usize),
    PayResources(usize),
}

pub(crate) trait MessageHandler {

    fn handle_message(&mut self, msg: UpdateMessage);
}

impl MessageHandler for SecCardGameApp {
    fn handle_message(&mut self, msg: UpdateMessage)  {
        match msg {
            UpdateMessage::SetResourceGain(res) => {
                self.game = self.game.set_resource_gain(Resources::new(res));
                self.input.next_res = res.to_string();
            }
            UpdateMessage::PayResources(res) => {
                self.game = self.game.pay_resources(&Resources::new(res));
                match self.game.action_status {
                    GameActionResult::Payed => {}
                    | GameActionResult::NothingPayed => {
                        self.input.pay_res = "0".to_string();
                        self.input.message = Message::None;
                    }
                    GameActionResult::NotEnoughResources => {
                        self.input.message =
                            Message::Warning("Not enough resources!".to_string());
                    }
                    _ => {}
                }

            }
        }
    }
}