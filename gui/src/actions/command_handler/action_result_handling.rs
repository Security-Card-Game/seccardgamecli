/// # Game Action Result handling
/// In here the current game action status is read and the UI messages are updated accordingly.
use crate::{Message, SecCardGameApp};
use game_lib::world::game::GameActionResult;

impl SecCardGameApp {
    pub(super) fn process_game_action_status(&mut self) {
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
                self.input.message = Message::Warning("Attack forced to be over".to_string());
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
                self.input.message = Message::Failure("Invalid Action!".to_string())
            }
            GameActionResult::Success => self.input.message = Message::None,
        }
    }
}
