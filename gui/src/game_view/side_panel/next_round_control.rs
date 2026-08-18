use egui::{RichText, Ui};

use game_lib::world::game::GameStatus;

use crate::game_view::actions::command::Command;
use crate::game_view::state::{GameViewState, Message};

impl GameViewState {
    pub(crate) fn next_round_controls(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        match &self.game.status {
            GameStatus::Finished(_) => {
                ui.label("Game ended");
            }
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                if board.turns_remaining > 0 && ui.button(RichText::new("Draw card").strong()).clicked() {
                    self.input.message = Message::None;
                    self.game = self.game.next_round();
                }
            }
        };
    }
}