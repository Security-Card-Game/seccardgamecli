use egui::{RichText, Ui};

pub(crate) struct LabelWithInputComponent {
    pub(crate) label: String,
    pub(crate) description: Option<String>,
    pub(crate) value: String,
}

impl LabelWithInputComponent {
    fn update(&mut self, new_value: String) {
        self.value = new_value;
    }

    pub(crate) fn draw_component<T>(&mut self, default: T, ui: &mut Ui)
    where
        T: std::str::FromStr + ToString,
    {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(&self.label);
                ui.add_space(5.0);
                let input = ui.add(egui::TextEdit::singleline(&mut self.value).desired_width(20.0));
                let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                if input.lost_focus() || enter_pressed {
                    let value = self.value.parse::<T>().unwrap_or(default);
                    self.update(value.to_string());
                }
            });
            if let Some(description) = &self.description {
                ui.add_space(5.0);
                ui.label(RichText::new(description).small());
            }
            ui.add_space(10.0);
        });
    }
}
