use egui::Context;
use egui::RichText;
use game_lib::world::deck::DeckComposition;
pub(crate) use crate::{AppEvent, InitViewState};
pub(crate) use crate::components::components::LabelWithInputComponent;

impl InitViewState {
    pub fn new() -> Self {
        InitViewState {
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

    pub fn draw_ui<F>(&mut self, event_callback: &mut F, ctx: &Context)
    where
        F: FnMut(AppEvent),
    {
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
           return if ui.button("Start Game").clicked() {
               let deck_composition = DeckComposition {
                   events: self.event_card_count.value.parse().unwrap_or(0),
                   attacks: self.attack_card_count.value.parse().unwrap_or(0),
                   oopsies: self.oopsie_card_count.value.parse().unwrap_or(0),
                   lucky: self.lucky_card_count.value.parse().unwrap_or(0),
                   evaluation: self.evaluation_card_count.value.parse().unwrap_or(0),
               };
               let grace_rounds = self.grace_rounds.value.parse().unwrap_or(0);
               event_callback(AppEvent::start_game(deck_composition, grace_rounds)
               )
           }
        });
    }
}
