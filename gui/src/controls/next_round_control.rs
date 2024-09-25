use egui::Ui;

use game_lib::world::game::GameStatus;

use crate::command_handler::Command;
use crate::{Message, SecCardGameApp};

impl SecCardGameApp {
    pub(crate) fn next_round_controls(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.label("Next round");
        ui.add_space(5.0);

        ui.label("Gain resources ");
        self.numeric_enter_component(
            ui,
            |game| &mut game.input.next_res,
            "Set",
            |val| {
                Command::SetResourceGain(val)
            },
        );

        ui.add_space(5.0);
        match &self.game.status {
            GameStatus::Finished(_) => {
                ui.label("Game ended");
            }
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                if board.turns_remaining > 0 && ui.button("Draw card").clicked() {
                    self.input.message = Message::None;
                    self.game = self.game.next_round();
                }
            }
        };
    }
}