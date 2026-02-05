use egui::{Align, Layout, RichText, Ui};

pub(crate) struct LabelWithInputComponent {
    pub(crate) label: String,
    pub(crate) description: Option<String>,
    pub(crate) value: String,
}

#[derive(Copy, Clone)]
pub(crate) struct LabelWithInputLayoutOptions {
    pub(crate) input_width: f32,
    pub(crate) max_width: f32,
}

impl Default for LabelWithInputLayoutOptions {
    fn default() -> Self {
        Self {
            input_width: 20.0,
            max_width: 200.0,
        }
    }
}

impl LabelWithInputComponent {
    pub fn update(&mut self, new_value: String) {
        self.value = new_value;
    }

    const CELL_HEIGHT: f32 = 18.0;
    const GAP: f32 = 4.0;

    pub(crate) fn draw_component<T>(
        &mut self,
        default: T,
        ui: &mut Ui,
        options: LabelWithInputLayoutOptions,
    ) where
        T: std::str::FromStr + ToString,
    {
        let width = if ui.available_width() < options.max_width {
            ui.available_width()
        } else {
            options.max_width
        };

        ui.vertical(|ui| {
            ui.set_width(width);
            let row_width = ui.available_width().min(options.max_width);

            ui.allocate_ui_with_layout(
                egui::Vec2::new(row_width, Self::CELL_HEIGHT),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = Self::GAP;
                    let label_width =
                        (ui.available_width() - options.input_width - ui.spacing().item_spacing.x)
                            .max(0.0);

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(
                            label_width - ui.spacing().item_spacing.x - options.input_width,
                            Self::CELL_HEIGHT,
                        ),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.label(&self.label);
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width(), Self::CELL_HEIGHT),
                        Layout::right_to_left(Align::Min),
                        |ui| {
                            let input = ui.add(
                                egui::TextEdit::singleline(&mut self.value)
                                    .desired_width(options.input_width),
                            );
                            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                            if input.lost_focus() || enter_pressed {
                                let value = self.value.parse::<T>().unwrap_or(default);
                                self.update(value.to_string());
                            }
                        },
                    );
                },
            );
            if let Some(description) = &self.description {
                ui.add_space(5.0);
                ui.label(RichText::new(description).small());
            }
            ui.add_space(10.0);
        });
    }
}

impl Into<u8> for LabelWithInputComponent {
    fn into(self) -> u8 {
        self.value.parse::<u8>().unwrap_or(0)
    }
}

impl Into<u8> for &LabelWithInputComponent {
    fn into(self) -> u8 {
        self.value.parse::<u8>().unwrap_or(0)
    }
}

impl Into<usize> for LabelWithInputComponent {
    fn into(self) -> usize {
        self.value.parse::<usize>().unwrap_or(0)
    }
}

impl Into<usize> for &LabelWithInputComponent {
    fn into(self) -> usize {
        self.value.parse::<usize>().unwrap_or(0)
    }
}
