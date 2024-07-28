use egui::Ui;

use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;

use crate::{Message, SecCardGameApp};

impl SecCardGameApp {
    pub(crate) fn tweak_control(&mut self, ui: &mut Ui) {
        ui.label("Tweaks");
        ui.add_space(5.0);

        ui.label("Multiply all fix costs by:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input.multiplier);
            ui.add_space(5.0);

            if ui.button("Set Multiplier").clicked() {
                let new_multiplier: isize = self.input.multiplier.parse().unwrap_or_else(|_| 1);
                if new_multiplier <= 0 {
                   self.input.message = Message::Failure("Invalid Action, must be > 0!".to_string());
                    return;
                }
                self.input.message = Message::None;
                self.game = self
                    .game
                    .set_fix_multiplier(ResourceFixMultiplier::new(new_multiplier.unsigned_abs()));
            }
        });
    }

}