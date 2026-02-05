use crate::components::components::{LabelWithInputComponent, LabelWithInputLayoutOptions};
use crate::{AppEvent, StartGameData, ViewState};
use eframe::emath::Align;
use egui::{ComboBox, Context, Layout, RichText, ScrollArea, Ui, Vec2};
use game_lib::cards::game_variants::scenario::Scenario;
use game_lib::cards::properties::description::Description;
use game_lib::file::repository::DeckLoader;
use game_lib::world::deck::{DeckComposition, GameVariantsRepository};
use game_lib::world::game::GameInitSettings;
use game_lib::world::reputation::Reputation;
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;
use game_setup::config::config::Config;
use std::rc::Rc;

struct DeckSettings {
    event_card_count: LabelWithInputComponent,
    attack_card_count: LabelWithInputComponent,
    oopsie_card_count: LabelWithInputComponent,
    lucky_card_count: LabelWithInputComponent,
    evaluation_card_count: LabelWithInputComponent,
    grace_rounds: LabelWithInputComponent,
}

struct ScenarioSettings {
    scenario_value: String,
    current_scenario: Option<Rc<Scenario>>,
    scenarios: Vec<Rc<Scenario>>,
}

pub struct InitViewState {
    deck_settings: DeckSettings,
    game_preset: GamePreset,
    scenario_settings: ScenarioSettings,
}

struct GamePreset {
    initial_resources: LabelWithInputComponent,
    initial_reputation: LabelWithInputComponent,
    initial_resource_gain: LabelWithInputComponent,
    initial_fix_multiplier: LabelWithInputComponent,
}

impl Default for GamePreset {
    fn default() -> Self {
        let default = GameInitSettings::default();

        GamePreset {
            initial_resources: LabelWithInputComponent {
                label: "Initial resources".to_string(),
                description: Some("The number of resources you start the game with.".to_string()),
                value: default.resources.value().to_string(),
            },
            initial_reputation: LabelWithInputComponent {
                label: "Initial reputation".to_string(),
                description: Some(
                    "The number of reputation points you start the game with. [0 - 100]"
                        .to_string(),
                ),
                value: default.reputation.value().to_string(),
            },
            initial_resource_gain: LabelWithInputComponent {
                label: "Initial resource gain".to_string(),
                description: Some("The number of resources you gain per turn.".to_string()),
                value: default.resource_gain.value().to_string(),
            },
            initial_fix_multiplier: LabelWithInputComponent {
                label: "Initial fix multiplier".to_string(),
                description: Some("The multiplier for fixing cards.".to_string()),
                value: default.fix_multiplier.value().to_string(),
            },
        }
    }
}

impl Default for DeckSettings {
    fn default() -> Self {
        DeckSettings {
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
            }
        }
    }
}

impl Into<GameInitSettings> for &GamePreset {
    fn into(self) -> GameInitSettings {
        let reputation: u8 = (&self.initial_reputation).into();

        GameInitSettings {
            resource_gain: Resources::new((&self.initial_resource_gain).into()),
            resources: Resources::new((&self.initial_resources).into()),
            fix_multiplier: ResourceFixMultiplier::new((&self.initial_fix_multiplier).into()),
            reputation: Reputation::new(reputation.min(100).into()),
        }
    }
}

impl Default for ScenarioSettings {
    fn default() -> Self {
        ScenarioSettings {
            scenarios: vec![],
            scenario_value: "None".to_string(),
            current_scenario: None,
        }
    }
}

impl InitViewState {
    pub fn new(config: &Config) -> Self {
        let scenarios = DeckLoader::create(&config.game_path).get_scenarios();
        InitViewState {
            scenario_settings: ScenarioSettings {
                scenarios,
                ..ScenarioSettings::default()
            },
            deck_settings: DeckSettings::default(),
            game_preset: GamePreset::default(),
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

    fn draw_game_deck_settings(&mut self, ui: &mut Ui) {
        let control_layout_options = LabelWithInputLayoutOptions {
            max_width: Self::LEFT_COL_WIDTH,
            ..LabelWithInputLayoutOptions::default()
        };
        ui.spacing_mut().item_spacing.y = Self::DEFAULT_SPACING_Y;
        ui.label(RichText::new("Game Deck Settings").strong());
        self.deck_settings
            .event_card_count
            .draw_component(0, ui, control_layout_options);
        self.deck_settings
            .attack_card_count
            .draw_component(0, ui, control_layout_options);
        self.deck_settings
            .oopsie_card_count
            .draw_component(0, ui, control_layout_options);
        self.deck_settings
            .lucky_card_count
            .draw_component(0, ui, control_layout_options);
        self.deck_settings
            .grace_rounds
            .draw_component(0, ui, control_layout_options);
        ui.add_space(Self::DEFAULT_SPACE_Y);
        ui.label(RichText::new("Experimental Settings").strong());
        self.deck_settings
            .evaluation_card_count
            .draw_component(0, ui, control_layout_options);
    }

    fn draw_scenario_selection(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Scenario").strong());
        let max_width = ui.available_width();

        let mut scenario_values = vec!["None"];
        scenario_values.append(
            &mut self
                .scenario_settings
                .scenarios
                .iter()
                .map(|scenario| &*scenario.title.value())
                .collect::<Vec<&str>>(),
        );

        let old_selection = self.scenario_settings.scenario_value.clone();

        ComboBox::new("scenarios", "Select Scenario")
            .width(max_width)
            .selected_text(self.scenario_settings.scenario_value.clone())
            .show_ui(ui, |ui| {
                for title in scenario_values {
                    ui.selectable_value(
                        &mut self.scenario_settings.scenario_value,
                        title.to_string(),
                        title,
                    );
                }
            });

        if self.scenario_settings.scenario_value == old_selection {
            return;
        };

        self.scenario_settings.current_scenario = self
            .scenario_settings
            .scenarios
            .iter()
            .find(|scenario| scenario.title.value() == &self.scenario_settings.scenario_value)
            .cloned();

        ui.add_space(Self::DEFAULT_SPACE_Y);

        let empty_description =
            Description::new("No description available. Select a scenario to see its description.");
        let description = self
            .scenario_settings
            .current_scenario
            .as_ref()
            .map(|scenario| &scenario.description)
            .unwrap_or(&empty_description);

        ScrollArea::vertical()
            .max_height(50.0)
            .max_width(max_width)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.set_width(ui.available_width()); // helps to wrap behave nicely
                ui.label(description.value()); // read-only, wraps by default
            });

        if let Some(scenario) = self.scenario_settings.current_scenario.as_ref() {
            self.game_preset
                .initial_resources
                .update(scenario.preset.resources.value().to_string());
            self.game_preset
                .initial_resource_gain
                .update(scenario.preset.resource_gain.value().to_string());
            self.game_preset
                .initial_reputation
                .update(scenario.preset.reputation.value().to_string());
            self.game_preset
                .initial_fix_multiplier
                .update(scenario.preset.multiplier.value().to_string());
        } else {
            let defaults = GamePreset::default();
            self.game_preset
                .initial_resources
                .update(defaults.initial_resources.value);
            self.game_preset
                .initial_resource_gain
                .update(defaults.initial_resource_gain.value);
            self.game_preset
                .initial_fix_multiplier
                .update(defaults.initial_fix_multiplier.value);
            self.game_preset
                .initial_reputation
                .update(defaults.initial_reputation.value);
        }
    }

    fn draw_game_preset(&mut self, ui: &mut Ui) {
        let control_layout_options = LabelWithInputLayoutOptions {
            max_width: Self::RIGHT_COL_WIDTH,
            input_width: 50.0,
            ..LabelWithInputLayoutOptions::default()
        };

        ui.label(RichText::new("Game Presets").strong());

        self.game_preset
            .initial_resources
            .draw_component(0, ui, control_layout_options);
        self.game_preset
            .initial_resource_gain
            .draw_component(0, ui, control_layout_options);
        self.game_preset
            .initial_reputation
            .draw_component(0, ui, control_layout_options);
        self.game_preset
            .initial_fix_multiplier
            .draw_component(0, ui, control_layout_options);
    }

    fn draw_start_button(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ui: &mut Ui) {
        if ui.button("Start Game").clicked() {
            let deck_composition = DeckComposition {
                events: (&self.deck_settings.event_card_count).into(),
                attacks: (&self.deck_settings.attack_card_count).into(),
                oopsies: (&self.deck_settings.oopsie_card_count).into(),
                lucky: (&self.deck_settings.lucky_card_count).into(),
                evaluation: (&self.deck_settings.evaluation_card_count).into(),
            };
            let grace_rounds = (&self.deck_settings.grace_rounds).into();

            let game_init_settings = (&self.game_preset).into();
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
                            self.draw_game_preset(ui);
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
                    self.draw_game_preset(ui);
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
