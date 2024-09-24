use egui::Context;

use super::SecCardGameApp;

impl SecCardGameApp {
    pub(crate) fn create_control_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .show_separator_line(true)
            .max_width(150.0)
            .show(ctx, |ui| {
                self.next_round_controls(ui);

                ui.add_space(15.0);

                self.resource_control(ui);

                ui.add_space(15.0);

                self.tweak_control(ui);

                ui.add_space(10.0);

                self.game_status_display(ui);
            });
    }

}