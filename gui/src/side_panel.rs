use egui::Context;

use crate::GameViewState;

mod tweak_control;
mod next_round_control;
mod game_status_display;
mod resource_control;
mod reputation_control;

impl GameViewState {
    pub(crate) fn create_side_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .show_separator_line(true)
            .max_width(150.0)
            .show(ctx, |ui| {
                self.next_round_controls(ui);

                ui.add_space(15.0);

                self.resource_control(ui);

                ui.add_space(15.0);

                self.reputation_control(ui);

                ui.add_space(30.0);

                self.tweak_control(ui);

                ui.add_space(10.0);

                self.game_status_display(ui);
            });
    }

}