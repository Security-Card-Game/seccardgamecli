use egui::{Context, RichText, Ui};

struct LabelValue {
    label: String,
    description: Option<String>,
    value: String,
}

impl LabelValue {
    fn update(&mut self, new_value: String) {
        self.value = new_value;
    }

    fn draw_component(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(&self.label);
                ui.add_space(5.0);
                let input = ui.add(egui::TextEdit::singleline(&mut self.value).desired_width(20.0));
                let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                if input.lost_focus() || enter_pressed {
                    let value = self.value.parse::<u8>().unwrap_or(0);
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

pub(crate) struct InitView {
    event_card_count: LabelValue,
    attack_card_count: LabelValue,
    oopsie_card_count: LabelValue,
    lucky_card_count: LabelValue,
    evaluation_card_count: LabelValue,
    grace_rounds: LabelValue,
}

impl InitView {
    pub fn new() -> Self {
        InitView {
            event_card_count: LabelValue {
                label: "Number of event cards".to_string(),
                description: None,
                value: "10".to_string(),
            },
            attack_card_count: LabelValue {
                label: "Number of attack cards".to_string(),
                description: None,
                value: "5".to_string(),
            },
            oopsie_card_count: LabelValue {
                label: "Number of oopsie cards".to_string(),
                description: None,
                value: "15".to_string(),
            },
            lucky_card_count: LabelValue {
                label: "Number of lucky Cards".to_string(),
                description: None,
                value: "5".to_string(),
            },
            evaluation_card_count: LabelValue {
                label: "Experimental: Evaluation Cards".to_string(),
                description: Some("The deck will be split into n + 1 parts and all parts except the first will contain an evaluation card. 0 disables them.".to_string()),
                value: "0".to_string(),
            },
            grace_rounds: LabelValue {
                label: "Grace Rounds".to_string(),
                description: Some("Number of turns after which attacks are possible".to_string()),
                value: "0".to_string(),
            },
        }
    }

    pub fn draw_ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("init_view").show(ui, |ui| {
                    ui.label(RichText::new("Game Deck Settings").strong());
                    ui.end_row();
                    self.event_card_count.draw_component(ui);
                    ui.end_row();
                    self.attack_card_count.draw_component(ui);
                    ui.end_row();
                    self.oopsie_card_count.draw_component(ui);
                    ui.end_row();
                    self.lucky_card_count.draw_component(ui);
                    ui.end_row();
                    self.grace_rounds.draw_component(ui);
                    ui.end_row();

                    ui.label(RichText::new("Experimental Settings").strong());
                    ui.end_row();
                    self.evaluation_card_count.draw_component(ui);
                    ui.end_row();
            });
        });
    }
}
