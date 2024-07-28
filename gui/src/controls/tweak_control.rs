use egui::Ui;

use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;

use crate::SecCardGameApp;

impl SecCardGameApp {
    pub(crate) fn tweak_control(&mut self, ui: &mut Ui) {
        ui.label("Tweaks");
        ui.add_space(5.0);

        ui.label("Multiply all fix costs by:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input.multiplier);
            ui.add_space(5.0);

            if ui.button("Set Multiplier").clicked() {
                let new_multiplier = self.input.multiplier.parse().unwrap_or_else(|_| 1);
                self.game = self
                    .game
                    .set_fix_multiplier(ResourceFixMultiplier::new(new_multiplier));
            }
        });
    }

}