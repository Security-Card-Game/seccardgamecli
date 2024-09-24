use game_lib::world::resources::Resources;
use crate::{Message, SecCardGameApp};

pub(crate) enum UpdateMessage {
    SetResourceGain(usize)
}

pub(crate) trait MessageHandler {

    fn handle_message(&mut self, msg: UpdateMessage) -> Message;
}

impl MessageHandler for SecCardGameApp {
    fn handle_message(&mut self, msg: UpdateMessage) -> Message {
        match msg {
            UpdateMessage::SetResourceGain(res) => {
                self.game = self.game.set_resource_gain(Resources::new(res));
                self.input.next_res = res.to_string();
                Message::None
            }
        }
    }
}