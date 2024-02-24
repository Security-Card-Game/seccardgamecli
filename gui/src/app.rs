use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;

use egui::{Align, FontId, Layout, Pos2, Ui, Visuals, Window};
use uuid::Uuid;
use game_lib::cards::model::{Card, CardTrait, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};
use game_lib::file::general::get_files_in_directory_with_filter;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct SeccardGameApp {
    // Example stuff:
    label: String,
    current_card: usize,
    total_cards: usize,
    cards: Vec<Card>,
    cards_to_display: HashMap<Uuid, CardContent>,
}

struct CardContent {
    id: Uuid,
    label: String,
    description: String,
    action: String,
    targets: Option<Vec<String>>,
    costs: Option<FixCost>,
}

impl SeccardGameApp {
    fn init(cards: Vec<Card>) -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
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
                let id = Uuid::new_v4();
                let card = match new_card {
                    Card::Event(c) => event_card_content(c),
                    Card::Incident(c) => incident_card_content(c),
                    Card::Oopsie(c) => oopsie_card_content(c),
                    Card::Lucky(c) => lucky_card_content(c)
                };
                self.cards_to_display.insert(id, card);
                self.current_card += 1;
            }
        }
    }

    fn create_card_window(mut ids_to_remove: &mut Vec<Uuid>, card: &&CardContent, ui: &mut Ui) {
        if ui.button("Close").clicked() {
            ids_to_remove.push(card.id)
        }
        ui.vertical(|ui| {
            ui.label(&card.description);
            ui.label("Action:");
            ui.label(&card.action);
            match &card.targets {
                None => {}
                Some(targets) => {
                    ui.label("Targets:");
                    let list = targets.join(", ");
                    ui.label(list);
                }
            }
            match &card.costs {
                None => {}
                Some(cost) => {
                    ui.label("Costs to fix:");
                    ui.label(format!("min: {}, max: {}", cost.min, cost.max));
                }
            };
        });
    }
}

impl SeccardGameApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        SeccardGameApp::init(Self::load_cards())
    }

    fn load_cards() -> Vec<Card> {
        let files = get_files_in_directory_with_filter("deck", ".json").expect("Deck");
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

impl eframe::App for SeccardGameApp {
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
                Window::new(&card.label)
                    .resizable(false)
                    .show(ctx, |ui| {
                        Self::create_card_window(&mut ids_to_remove, &card, ui);
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
        label: card.title,
        description: card.description,
        action: card.action,
        targets: None,
        costs: None,
    }
}



fn configure_text_styles(ctx: &egui::Context) {
    use egui::FontFamily::Proportional;
    use egui::TextStyle::*;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(12.0, Proportional)),
        (Body, FontId::new(10.0, Proportional)),
        (Monospace, FontId::new(9.0, Proportional)),
        (Button, FontId::new(10.5, Proportional)),
        (Small, FontId::new(8.0, Proportional)),
    ]
        .into();
    ctx.set_style(style);
}