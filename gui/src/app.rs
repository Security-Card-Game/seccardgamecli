use std::collections::HashMap;

use crate::card::{to_ui_deck, CardContent};
use crate::card_window::display_card;
use egui::{Align, Color32, Context, Layout, RichText, Ui, Window};
use egui::WidgetType::TextEdit;
use game_lib::cards::model::Card;
use uuid::Uuid;

pub struct SecCardGameApp {
    resources: usize,
    current_card: usize,
    total_cards: usize,
    cards: Vec<CardContent>,
    cards_to_display: HashMap<Uuid, CardContent>,
    resources_per_round: usize,
    input: Input
}

struct Input {
    next_res: String,
    pay_res: String,
    error: Option<String>,
}

impl SecCardGameApp {
    fn init(deck: Vec<CardContent>) -> Self {
        Self {
            resources: 0,
            resources_per_round: 5,
            current_card: 0,
            total_cards: deck.len(),
            cards: deck,
            cards_to_display: HashMap::new(),
            input: Input {
                next_res: "5".to_owned(),
                pay_res: "0".to_owned(),
                error: None,
            }
        }
    }

    fn refresh_cards(&mut self, ctx: &Context, ui: &mut Ui) {
        let mut ids_to_remove = vec![];
        for card in self.cards_to_display.values() {
            display_card(card, |id| ids_to_remove.push(id), ctx, ui);
        }

        for id in &ids_to_remove {
            self.cards_to_display.remove(id);
        }
    }

    fn create_menu_bar(ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.add_space(16.0);
                egui::gui_zoom::zoom_menu_buttons(ui);
            });
        });
    }

    fn crete_control_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.next_round_controls(ui);

                ui.add_space(15.0);

                self.resource_control(ui);

                ui.add_space(5.0);
                ui.label(format!("Cards {}/{}", self.current_card, self.total_cards));

                ui.add_space(20.0);
                match &self.input.error {
                    None => {}
                    Some(e) => { ui.label(RichText::new(e).color(Color32::RED)); }
                }
            });
    }

    fn resource_control(&mut self, ui: &mut Ui) {
        ui.label("Resources");
        ui.add_space(5.0);
        ui.label(format!("{} resources available", self.resources));
        ui.text_edit_singleline(&mut self.input.pay_res);
        ui.add_space(5.0);

        if (ui.button("Pay").clicked()) {
            let to_pay = self.input.pay_res.parse().unwrap_or_else(|_| 0);
            if (to_pay > self.resources) {
                self.input.error = Some("No money!".to_string())
            } else {
                self.resources -= to_pay;
                self.input.pay_res = "0".to_string();
                self.input.error = None;
            }
        };
    }

    fn next_round_controls(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.label("Next round");
        ui.add_space(5.0);
        ui.label("Gain resources ");
        let res = ui.add(egui::TextEdit::singleline(&mut self.input.next_res).interactive(true));
        if res.lost_focus() {
            self.resources_per_round = self.input.next_res.parse().unwrap_or_else(|_| self.resources_per_round);
        }

        ui.add_space(5.0);
        if self.current_card < self.total_cards && ui.button("Draw card").clicked() {
            self.add_card_to_display();
            self.resources += self.resources_per_round;
            self.current_card += 1;
            self.input.error = None;
        }
        if self.current_card == self.total_cards {
            ui.label("Game ended");
        }
    }
}

impl SecCardGameApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, deck: Vec<Card>) -> Self {
        let ui_deck = to_ui_deck(deck);
        SecCardGameApp::init(ui_deck)
    }

    fn add_card_to_display(&mut self) {
        let card = self.cards.pop();
        match card {
            None => log::error!("Could not draw card!"),
            Some(c) => {
                self.cards_to_display.insert(c.id, c);
            }
        }
    }
}

impl eframe::App for SecCardGameApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::create_menu_bar(ctx);

        self.crete_control_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.refresh_cards(ctx, ui);
        });
    }
}
