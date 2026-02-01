use std::rc::Rc;
use crate::components::components::LabelWithInputComponent;
use crate::{AppEvent, ViewState};
use egui::{Context, RichText, ComboBox};
use game_lib::cards::game_variants::scenario::Scenario;
use game_lib::file::repository::DeckLoader;
use game_lib::world::deck::{DeckComposition, GameVariantsRepository};
use game_setup::config::config::Config;

pub(crate) struct InitViewState {
    event_card_count: LabelWithInputComponent,
    attack_card_count: LabelWithInputComponent,
    oopsie_card_count: LabelWithInputComponent,
    lucky_card_count: LabelWithInputComponent,
    evaluation_card_count: LabelWithInputComponent,
    grace_rounds: LabelWithInputComponent,
    scenario_value: String,
    scenarios: Vec<Rc<Scenario>>,
}

impl InitViewState {
    pub fn new(config: &Config) -> Self {
        let scenarios = DeckLoader::create(&config.game_path).get_scenarios();
        InitViewState {
            scenarios,
            scenario_value: "None".to_string(),
            event_card_count: LabelWithInputComponent {
                label: "Number of event cards".to_string(),
                description: None,
                value: "10".to_string(),
            },
            attack_card_count: LabelWithInputComponent {
                label: "Number of attack cards".to_string(),
                description: None,
                value: "5".to_string(),
            },
            oopsie_card_count: LabelWithInputComponent {
                label: "Number of oopsie cards".to_string(),
                description: None,
                value: "15".to_string(),
            },
            lucky_card_count: LabelWithInputComponent {
                label: "Number of lucky cards".to_string(),
                description: None,
                value: "5".to_string(),
            },
            evaluation_card_count: LabelWithInputComponent {
                label: "Experimental: Evaluation cards".to_string(),
                description: Some("The deck will be split into n + 1 parts and all parts except the first will contain an evaluation card. 0 disables them.".to_string()),
                value: "0".to_string(),
            },
            grace_rounds: LabelWithInputComponent {
                label: "Grace rounds".to_string(),
                description: Some("Number of turns after which attacks are possible".to_string()),
                value: "6".to_string(),
            },
        }
    }
}
impl ViewState for InitViewState {
    fn draw_ui(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("init_view").show(ui, |ui| {
                ui.label(RichText::new("Game Deck Settings").strong());
                ui.end_row();
                self.event_card_count.draw_component(0, ui);
                ui.end_row();
                self.attack_card_count.draw_component(0, ui);
                ui.end_row();
                self.oopsie_card_count.draw_component(0, ui);
                ui.end_row();
                self.lucky_card_count.draw_component(0, ui);
                ui.end_row();
                self.grace_rounds.draw_component(0, ui);
                ui.end_row();

                ui.label(RichText::new("Experimental Settings").strong());
                ui.end_row();
                self.evaluation_card_count.draw_component(0, ui);
                ui.end_row();
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let mut scenario_values = vec!["None"];
                scenario_values.append(&mut self.scenarios.iter().map(|scenario| &*scenario.title.value()).collect::<Vec<&str>>());
                ComboBox::new("scenarios", "Select Scenario")
                    .width(300.0)
                    .selected_text(self.scenario_value.clone())
                    .show_ui(ui, |ui| {
                        for title in  scenario_values {
                            ui.selectable_value(&mut self.scenario_value, title.to_string(), title);
                        }
                    });

                let current_scenario = self.scenarios.iter().find(|scenario| scenario.title.value() == &self.scenario_value);
                if let Some(scenario) = current_scenario {
                    let mut content = format!("{:?}", scenario);
                    ui.text_edit_multiline(&mut content);
                }
            });


            return if ui.button("Start Game").clicked() {
                let deck_composition = DeckComposition {
                    events: self.event_card_count.value.parse().unwrap_or(0),
                    attacks: self.attack_card_count.value.parse().unwrap_or(0),
                    oopsies: self.oopsie_card_count.value.parse().unwrap_or(0),
                    lucky: self.lucky_card_count.value.parse().unwrap_or(0),
                    evaluation: self.evaluation_card_count.value.parse().unwrap_or(0),
                };
                let grace_rounds = self.grace_rounds.value.parse().unwrap_or(0);
                app_event_callback(AppEvent::start_game(deck_composition, grace_rounds))
            };
        });
    }
}
