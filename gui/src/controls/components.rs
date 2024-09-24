use egui::Ui;
use crate::controls::messageing::{MessageHandler, UpdateMessage};
use crate::SecCardGameApp;

impl SecCardGameApp {
    pub(crate) fn numeric_enter_component<T, B, F>(&mut self,
                                     ui: &mut Ui,
                                     mut backing_field: B,
                                     button_label: &str,
                                     on_click_message: F
    )
    where
        T: Default + std::str::FromStr,
        B: FnMut(&mut SecCardGameApp) -> &mut String,
        F: FnOnce(T) -> UpdateMessage,
    {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(backing_field(self));
            ui.add_space(5.0);

            if ui.button(button_label).clicked() {
                let value: T = backing_field(self).parse().unwrap_or(T::default());
                let msg = on_click_message(value);
                self.handle_message(msg);
            };
            ui.add_space(2.0)
        });
    }
}
