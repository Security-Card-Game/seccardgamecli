use std::collections::HashMap;

use crate::card::{
    event_card_content, incident_card_content, lucky_card_content, oopsie_card_content, CardContent,
};
use egui::{Align, Layout, RichText, Ui, Window};
use uuid::Uuid;
use game_lib::cards::model::Card;

pub struct SecCardGameApp {
    current_card: usize,
    total_cards: usize,
    cards: Vec<Card>,
    cards_to_display: HashMap<Uuid, CardContent>,
}

impl SecCardGameApp {
    fn init(cards: Vec<Card>) -> Self {
        Self {
            current_card: 0,
            total_cards: cards.len(),
            cards,
            cards_to_display: HashMap::new(),
        }
    }

    fn add_card_to_display(&mut self) {
        match self.cards.pop() {
            None => (),
            Some(new_card) => {
                let card = match new_card {
                    Card::Event(c) => event_card_content(c),
                    Card::Incident(c) => incident_card_content(c),
                    Card::Oopsie(c) => oopsie_card_content(c),
                    Card::Lucky(c) => lucky_card_content(c),
                };
                self.cards_to_display.insert(card.id, card);
                self.current_card += 1;
            }
        }
    }

    fn create_card_window(ids_to_remove: &mut Vec<Uuid>, card: &&CardContent, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let header_color = if ui.visuals().dark_mode {
                    card.dark_color
                } else {
                    card.light_color
                };
                let header = RichText::new(&card.label).color(header_color).heading();
                ui.label(header);
                if ui.button("X").clicked() {
                    ids_to_remove.push(card.id)
                }
            });
            ui.add_space(5.0);
            ui.label(&card.description);
            ui.add_space(2.0);
            let name = RichText::new("Action: ").strong();
            ui.label(name);
            ui.label(&card.action);
            match &card.targets {
                None => {}
                Some(targets) => {
                    let name = RichText::new("Targets: ").strong();
                    ui.label(name);
                    let list = targets.join(", ");
                    ui.label(list);
                }
            }
            ui.add_space(2.0);
            match &card.costs {
                None => {}
                Some(cost) => {
                    let name = RichText::new("Cost to fix: ").strong();
                    ui.label(name);
                    ui.label(format!("{} to {}", cost.min, cost.max));
                }
            };
        });
    }
}

impl SecCardGameApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, deck: Vec<Card>) -> Self {
        let mut deck_copy = deck.clone();
        deck_copy.reverse();
        SecCardGameApp::init(deck_copy)
    }
}

impl eframe::App for SecCardGameApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

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
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                ui.label(format!("Cards {}/{}", self.current_card, self.total_cards));
                if self.current_card < self.total_cards && ui.button("Draw card").clicked() {
                    self.add_card_to_display();
                }
                if self.current_card == self.total_cards {
                    ui.label("Game ended");
                }
                ui.add_space(20.0);
                egui::gui_zoom::zoom_menu_buttons(ui);
            });

            let mut ids_to_remove = vec![];
            for card in self.cards_to_display.values() {
                Window::new(card.id.to_string())
                    .title_bar(false)
                    .resizable(false)
                    .collapsible(false)
                    .default_width(200.0)
                    .show(ctx, |ui| {
                        Self::create_card_window(&mut ids_to_remove, &card, ui)
                    });
            }

            for id in &ids_to_remove {
                self.cards_to_display.remove(id);
            }
        });
    }
}
