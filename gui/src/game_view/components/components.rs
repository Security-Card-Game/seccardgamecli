use crate::game_view::actions::command::Command;
use crate::GameViewState;
use egui::Ui;

impl GameViewState {
    pub(crate) fn numeric_enter_component<T, B, F>(&mut self,
                                     ui: &mut Ui,
                                     mut backing_field: B,
                                     button_label: &str,
                                     on_click_message: F
    )
    where
        T: Default + std::str::FromStr,
        B: FnMut(&mut GameViewState) -> &mut String,
        F: FnOnce(T) -> Command,
    {
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(backing_field(self)).desired_width(20.0));
            ui.add_space(5.0);

            if ui.button(button_label).clicked() {
                let value: T = backing_field(self).parse().unwrap_or(T::default());
                let cmd = on_click_message(value);
                self.command = Some(cmd);
            };
            ui.add_space(2.0)
        });
    }
}