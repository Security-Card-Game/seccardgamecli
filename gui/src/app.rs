use std::collections::HashMap;

use egui::{Align, FontId, Layout, Pos2, Visuals, Window};
use uuid::Uuid;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct SeccardGameApp {
    // Example stuff:
    label: String,
    current_card: u16,
    total_cards: u16,
    cards: HashMap<Uuid, CardContent>,
}

struct CardContent {
    id: Uuid,
    label: String,
    action: String,
}

impl Default for SeccardGameApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            current_card: 0,
            total_cards: 10,
            cards: HashMap::new(),
        }
    }
}

impl SeccardGameApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Default::default()
    }
}

impl eframe::App for SeccardGameApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        configure_text_styles(ctx);
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
                    let id = Uuid::new_v4();
                    let card = CardContent {
                        id,
                        label: format!("Test {}", id),
                        action: "Action".to_string(),
                    };
                    self.cards.insert(id, card);
                    self.current_card += 1;
                }
                if self.current_card == self.total_cards {
                    ui.label("Game ended");
                }
                ui.add_space(20.0);
                egui::gui_zoom::zoom_menu_buttons(ui);
            });

            let mut ids_to_remove = vec![];
            for card in self.cards.values() {
                Window::new(&card.label).resizable(false).show(ctx, |ui| {
                    ui.label(&card.action);
                    if ui.button("Close").clicked() {
                        ids_to_remove.push(card.id)
                    }
                });
            }

            for id in &ids_to_remove {
                self.cards.remove(id);
            }
        });
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