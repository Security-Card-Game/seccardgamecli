use std::collections::HashMap;
use std::fs;

use egui::{Align, Color32, Layout, RichText, Ui, Window};
use game_lib::cards::model::{
    Card, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard,
};
use game_lib::file::general::get_files_in_directory_with_filter;
use uuid::Uuid;

pub struct SecCardGameApp {
    current_card: usize,
    total_cards: usize,
    cards: Vec<Card>,
    cards_to_display: HashMap<Uuid, CardContent>,
}

struct CardContent {
    id: Uuid,
    dark_color: Color32,
    light_color: Color32,
    label: String,
    description: String,
    action: String,
    targets: Option<Vec<String>>,
    costs: Option<FixCost>,
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
                } else  {
                    card.light_color
                };
                let header = RichText::new(&card.label)
                    .color(header_color)
                    .heading();
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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        SecCardGameApp::init(Self::load_cards())
    }

    fn load_cards() -> Vec<Card> {
        let files =
            get_files_in_directory_with_filter("deck", ".json")
                .expect("Deck");
        let mut cards = vec![];
        for file in files {
            let content = fs::read_to_string(file).expect("file content");
            let card = serde_json::from_str::<Card>(content.as_str()).unwrap();
            cards.push(card)
        }
        cards.reverse();
        cards
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

fn event_card_content(card: EventCard) -> CardContent {
    let id = Uuid::new_v4();
    CardContent {
        id,
        dark_color: Color32::LIGHT_BLUE,
        light_color: Color32::DARK_BLUE,
        label: card.title,
        description: card.description,
        action: card.action,
        targets: None,
        costs: None,
    }
}

fn incident_card_content(card: IncidentCard) -> CardContent {
    let id = Uuid::new_v4();
    CardContent {
        id,
        dark_color: Color32::LIGHT_RED,
        light_color: Color32::DARK_RED,
        label: card.title,
        description: card.description,
        action: card.action,
        targets: Some(card.targets),
        costs: None,
    }
}

fn oopsie_card_content(card: OopsieCard) -> CardContent {
    let id = Uuid::new_v4();
    CardContent {
        id,
        dark_color: Color32::YELLOW,
        light_color: Color32::DARK_GRAY,
        label: card.title,
        description: card.description,
        action: card.action,
        targets: Some(card.targets),
        costs: Some(card.fix_cost),
    }
}

fn lucky_card_content(card: LuckyCard) -> CardContent {
    let id = Uuid::new_v4();
    CardContent {
        id,
        dark_color: Color32::GREEN,
        light_color: Color32::DARK_GREEN,
        label: card.title,
        description: card.description,
        action: card.action,
        targets: None,
        costs: None,
    }
}
