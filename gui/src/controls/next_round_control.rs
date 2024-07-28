use egui::Ui;

use game_lib::world::game::GameStatus;
use game_lib::world::resources::Resources;

use crate::{Message, SecCardGameApp};

impl SecCardGameApp {
    pub(crate) fn next_round_controls(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.label("Next round");
        ui.add_space(5.0);

        ui.label("Gain resources ");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input.next_res);
            ui.add_space(5.0);

            if ui.button("Set ").clicked() {
                let new_gain = self.input.next_res.parse().unwrap_or(0usize);

                self.game = self.game.set_resource_gain(Resources::new(new_gain));
                self.input.next_res = self.game.resource_gain.value().to_string();
            }
        });

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