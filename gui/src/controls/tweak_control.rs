use egui::Ui;

use crate::command_handler::Command;
use crate::SecCardGameApp;

impl SecCardGameApp {
    pub(crate) fn tweak_control(&mut self, ui: &mut Ui) {
        ui.label("Tweaks");
        ui.add_space(5.0);

        ui.label("Multiply all fix costs by:");

        self.numeric_enter_component(
            ui,
            |game| &mut game.input.multiplier,
            "Set",
            |value| Command::SetMultiplier(value),
        );
    }
}
