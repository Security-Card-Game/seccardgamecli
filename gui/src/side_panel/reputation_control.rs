use egui::{RichText, Ui};

use game_lib::world::board::Board;
use game_lib::world::game::GameStatus;
use crate::actions::command::Command;
use crate::SecCardGameApp;

impl SecCardGameApp {
    pub(crate) fn reputation_control(&mut self, ui: &mut Ui) {
        ui.label("Reputation");
        ui.add_space(5.0);
        match &self.game.status.clone() {
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                self.reputation_control_game_in_progress(ui, board);
            }
            GameStatus::Finished(board) => {
                reputation_control_game_ended(ui, board);
            }
        }
    }

    fn reputation_control_game_in_progress(&mut self, ui: &mut Ui, board: &Board) {
        create_reputation_label(board, ui);

        self.numeric_enter_component(
            ui,
            |game| &mut game.input.inc_reputation,
            "Increase",
            |value| Command::IncreaseReputation(value),
        );

        self.numeric_enter_component(
            ui,
            |game| &mut game.input.dec_reputation,
            "Decrease",
            |value| Command::DecreaseReputation(value),
        );

    }
}

fn reputation_control_game_ended(ui: &mut Ui, board: &Board) {
    create_reputation_label(board, ui);
}

// helper function to create rich text for reputation amount,
// will be used in match arms for GameStatus
fn create_reputation_label(board: &Board, ui: &mut Ui) {
    ui.label(RichText::new(board.current_reputation.to_string()).strong());
}
