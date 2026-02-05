use crate::GameViewState;
use egui::{Context, RichText, Ui};

mod game_status_display;
mod next_round_control;
mod reputation_control;
mod resource_control;
mod tweak_control;

impl GameViewState {
    pub(crate) fn create_side_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .show_separator_line(true)
            .max_width(150.0)
            .show(ctx, |ui| {
                self.next_round_controls(ui);

                ui.add_space(15.0);

                self.draw_scenario(ui);
                ui.add_space(5.0);
                self.draw_goals(ui);
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

    fn draw_goals(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Game Goals").strong());
            ui.label(format!("Min Resources: {}", self.game_goals.min_resources));
            ui.label(format!(
                "Min Reputation: {}",
                self.game_goals.min_reputation
            ));
        });
    }

    fn draw_scenario(&mut self, ui: &mut Ui) {
        if let Some(scenario) = &self.scenario {
            ui.vertical(|ui| {
                ui.label(RichText::new("Scenario").strong());
                ui.label(scenario.title.value())
                    .on_hover_text(scenario.description.value());
            });
        }
    }
}
