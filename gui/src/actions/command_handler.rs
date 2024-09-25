/// # Command Handler to handle Game Actions
/// ## Concept
/// The GUI state (SecCardGameApp) contains a shareable, mutable reference containing a command
/// to be executed every update cycle. Each component (e.g. buttons) can update this field when it is
/// interacted with. As soon as a component is triggered the update cycle is triggered as well.
/// Inside the update cycle `process_command` is called. This command delegates the handling of it
/// to various methods which in turn mutate the GUI state (e.g. remove a card from the board). Once
/// this is done, the command is set to none and the update cycle is completed.

use crate::{Message, SecCardGameApp};
use crate::actions::command::Command;

pub(crate) trait CommandHandler {
    fn process_command(&mut self);
}


impl CommandHandler for SecCardGameApp {

    fn process_command(&mut self) {
        if let Some(cmd) = &self.command.clone() {
            self.handle_command(cmd);
        }
    }
}
pub mod card;
pub mod control_panel;
pub mod action_result_handling;

impl SecCardGameApp {
    fn handle_command(&mut self, msg: &Command) {
        self.input.message = Message::None;

        match msg {
            Command::SetResourceGain(res) => self.handle_set_resource_gain(res.clone()),
            Command::PayResources(res) => self.handle_pay_resources(res.clone()),
            Command::SetMultiplier(m) => self.handle_set_multiplier(m.clone()),
            Command::CloseCard(card_id) => self.handle_card_closed(card_id.clone()),
            Command::DeactivateCard(card_id) => self.handle_deactivate_card(card_id.clone()),
            Command::ActivateCard(card_id) => self.handle_activate_card(card_id.clone())
        }

        self.reset_command(); // removes the command from the state after it is executed
        self.process_game_action_status(); // takes care of displaying results of actions in the UI
    }

    fn reset_command(&mut self) {
        self.command = None
    }
}
