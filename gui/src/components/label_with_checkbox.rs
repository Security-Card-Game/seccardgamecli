use egui::{Align, Layout, RichText, Ui};
pub(crate) struct LabelWithCheckboxComponent {
    pub(crate) label: String,
    pub(crate) description: Option<String>,
    pub(crate) value: bool,
}

#[derive(Copy, Clone)]
pub(crate) struct LabelWithCheckboxOptions {
    pub(crate) input_width: f32,
    pub(crate) max_width: f32,
}

impl Default for LabelWithCheckboxOptions {
    fn default() -> Self {
        Self {
            input_width: 25.0,
            max_width: 200.0,
        }
    }
}

impl LabelWithCheckboxComponent {
    pub fn update(&mut self, new_value: bool) {
        self.value = new_value;
    }

    const CELL_HEIGHT: f32 = 18.0;
    const GAP: f32 = 4.0;

    pub(crate) fn draw_component(
        &mut self,
        default: bool,
        ui: &mut Ui,
        options: LabelWithCheckboxOptions,
    ) {
        self.value = default;
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
                           ui.add(
                                egui::Checkbox::new(&mut self.value, "")
                            );
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

