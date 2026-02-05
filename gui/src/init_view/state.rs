use crate::components::components::{LabelWithInputComponent, LabelWithInputLayoutOptions};
use crate::{AppEvent, StartGameData, ViewState};
use eframe::emath::Align;
use egui::{ComboBox, Context, Layout, RichText, ScrollArea, Ui, Vec2};
use game_lib::cards::game_variants::scenario::Scenario;
use game_lib::cards::properties::description::Description;
use game_lib::file::repository::DeckLoader;
use game_lib::world::deck::{DeckComposition, GameVariantsRepository};
use game_lib::world::game::GameInitSettings;
use game_setup::config::config::Config;
use std::rc::Rc;

pub(crate) struct InitViewState {
    event_card_count: LabelWithInputComponent,
    attack_card_count: LabelWithInputComponent,
    oopsie_card_count: LabelWithInputComponent,
    lucky_card_count: LabelWithInputComponent,
    evaluation_card_count: LabelWithInputComponent,
    grace_rounds: LabelWithInputComponent,
    scenario_value: String,
    current_scenario: Option<Rc<Scenario>>,
    scenarios: Vec<Rc<Scenario>>,
}

impl InitViewState {
    pub fn new(config: &Config) -> Self {
        let scenarios = DeckLoader::create(&config.game_path).get_scenarios();
        InitViewState {
            scenarios,
            scenario_value: "None".to_string(),
            current_scenario: None,
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

impl InitViewState {
    const GAP: f32 = 20.0;
    const DEFAULT_SPACING_Y: f32 = 5.0;
    const DEFAULT_SPACE_Y: f32 = 10.0;
    const MARGIN_TB: f32 = 10.0;

    const LEFT_COL_WIDTH: f32 = 250.0; // game settings + start button
    const RIGHT_COL_WIDTH: f32 = 400.0; // scenario selection
    const CONTENT_MAX_WIDTH: f32 = Self::LEFT_COL_WIDTH + Self::GAP + Self::RIGHT_COL_WIDTH;

    fn draw_game_deck_settings(&mut self, ui: &mut egui::Ui) {
        let control_layout_options = LabelWithInputLayoutOptions {
            max_width: Self::LEFT_COL_WIDTH,
            ..LabelWithInputLayoutOptions::default()
        };
        ui.spacing_mut().item_spacing.y = Self::DEFAULT_SPACING_Y;
        ui.label(RichText::new("Game Deck Settings").strong());
        self.event_card_count
            .draw_component(0, ui, control_layout_options);
        self.attack_card_count
            .draw_component(0, ui, control_layout_options);
        self.oopsie_card_count
            .draw_component(0, ui, control_layout_options);
        self.lucky_card_count
            .draw_component(0, ui, control_layout_options);
        self.grace_rounds
            .draw_component(0, ui, control_layout_options);
        ui.add_space(Self::DEFAULT_SPACE_Y);
        ui.label(RichText::new("Experimental Settings").strong());
        self.evaluation_card_count
            .draw_component(0, ui, control_layout_options);
    }

    fn draw_scenario_selection(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Scenario").strong());
        let max_width = ui.available_width();

        let mut scenario_values = vec!["None"];
        scenario_values.append(
            &mut self
                .scenarios
                .iter()
                .map(|scenario| &*scenario.title.value())
                .collect::<Vec<&str>>(),
        );

        ComboBox::new("scenarios", "Select Scenario")
            .width(max_width)
            .selected_text(self.scenario_value.clone())
            .show_ui(ui, |ui| {
                for title in scenario_values {
                    ui.selectable_value(&mut self.scenario_value, title.to_string(), title);
                }
            });

        self.current_scenario = self
            .scenarios
            .iter()
            .find(|scenario| scenario.title.value() == &self.scenario_value)
            .cloned();

        ui.add_space(Self::DEFAULT_SPACE_Y);

        let empty_description =
            Description::new("No description available. Select a scenario to see its description.");
        let description = self
            .current_scenario
            .as_ref()
            .map(|scenario| &scenario.description)
            .unwrap_or(&empty_description);

        ScrollArea::vertical()
            .max_height(50.0)
            .max_width(max_width)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.set_width(ui.available_width()); // helps wrapping behave nicely
                ui.label(description.value()); // read-only, wraps by default
            });
    }

    fn draw_start_button(
        &mut self,
        app_event_callback: &mut dyn FnMut(AppEvent),
        ui: &mut egui::Ui,
    ) {
        if ui.button("Start Game").clicked() {
            let deck_composition = DeckComposition {
                events: self.event_card_count.value.parse().unwrap_or(0),
                attacks: self.attack_card_count.value.parse().unwrap_or(0),
                oopsies: self.oopsie_card_count.value.parse().unwrap_or(0),
                lucky: self.lucky_card_count.value.parse().unwrap_or(0),
                evaluation: self.evaluation_card_count.value.parse().unwrap_or(0),
            };
            let grace_rounds = self.grace_rounds.value.parse().unwrap_or(0);

            let game_init_settings = if let Some(selected_scenario) = self.current_scenario.as_ref()
            {
                selected_scenario.preset.into()
            } else {
                GameInitSettings::default()
            };
            let start_game_data = StartGameData {
                deck_composition,
                grace_rounds,
                game_init_settings,
            };

            app_event_callback(AppEvent::start_game(start_game_data))
        };
    }

    fn draw_two_col_layout(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.allocate_ui(Vec2::new(Self::CONTENT_MAX_WIDTH, 0.0), |ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.spacing_mut().item_spacing.x = Self::GAP;

                    ui.allocate_ui_with_layout(
                        Vec2::new(Self::LEFT_COL_WIDTH, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.set_width(Self::LEFT_COL_WIDTH);
                            self.draw_game_deck_settings(ui);
                        },
                    );

                    ui.allocate_ui_with_layout(
                        Vec2::new(Self::RIGHT_COL_WIDTH, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.set_width(Self::RIGHT_COL_WIDTH);
                            self.draw_scenario_selection(ui);
                            ui.add_space(Self::DEFAULT_SPACE_Y);
                            self.draw_start_button(app_event_callback, ui);
                        },
                    );
                });
            });
        });
    }

    fn draw_single_col_layout(
        &mut self,
        app_event_callback: &mut dyn FnMut(AppEvent),
        ui: &mut Ui,
    ) {
        ui.vertical(|ui| {
            let col_width = ui.available_width();

            ui.allocate_ui_with_layout(
                Vec2::new(col_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing.x = Self::DEFAULT_SPACE_Y;

                    self.draw_game_deck_settings(ui);
                    self.draw_scenario_selection(ui);
                    self.draw_start_button(app_event_callback, ui);
                },
            );
        });
    }
}
impl ViewState for InitViewState {
    fn draw_ui(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let needs_single_col = ui.available_width() < Self::CONTENT_MAX_WIDTH;
            ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(Self::MARGIN_TB);
                if needs_single_col {
                    self.draw_single_col_layout(app_event_callback, ui);
                } else {
                    self.draw_two_col_layout(app_event_callback, ui);
                }
            });
        });
    }
}
