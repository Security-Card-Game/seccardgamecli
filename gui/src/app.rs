use std::collections::HashMap;

use crate::card::{to_ui_deck, CardContent};
use crate::card_window::display_card;
use egui::{Align, Context, Layout, RichText, Ui, Window};
use game_lib::cards::model::Card;
use uuid::Uuid;

pub struct SecCardGameApp {
    current_card: usize,
    total_cards: usize,
    cards: Vec<CardContent>,
    cards_to_display: HashMap<Uuid, CardContent>,
}

impl SecCardGameApp {
    fn init(deck: Vec<CardContent>) -> Self {
        Self {
            current_card: 0,
            total_cards: deck.len(),
            cards: deck,
            cards_to_display: HashMap::new(),
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

        egui::SidePanel::left("contol_panel")
            .show(ctx, |ui| {
                ui.add_space(5.0);
                if self.current_card < self.total_cards && ui.button("Draw card").clicked() {
                    self.add_card_to_display();
                    self.current_card += 1;
                }
                if self.current_card == self.total_cards {
                    ui.label("Game ended");
                }
                ui.add_space(5.0);
                ui.label(format!("Cards {}/{}", self.current_card, self.total_cards));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.refresh_cards(ctx, ui);
        });
    }
}
