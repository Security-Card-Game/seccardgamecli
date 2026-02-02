use egui::Context;

use super::{AppEvent, GameViewState, SecCardGameApp};
use crate::init_view::state::InitViewState;
use game_lib::world::deck::Deck;
use game_lib::world::game::{Game, GameInitSettings};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;
use game_setup::config::config::Config;
use game_setup::creation::create::create_deck;

impl SecCardGameApp {
    fn init(config: Config) -> Self {
        Self {
            active_view: Box::new(InitViewState::new(&config)),
            last_event: None,
            config,
        }
    }
    /// Called once before the first frame.
    pub fn new_with_deck(cc: &eframe::CreationContext<'_>, deck: Deck, config: Config) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.4);
        Self {
            active_view: Self::create_game_view_state(deck),
            last_event: None,
            config,
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.4);
        SecCardGameApp::init(config)
    }

    fn create_game_view_state(deck: Deck) -> Box<GameViewState> {
        let game = Game::create(deck, GameInitSettings::default());
        Box::new(GameViewState::new(game))
    }
}

impl eframe::App for SecCardGameApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.handle_app_event();

        Self::create_menu_bar(ctx);

        let mut event_publisher = |event| self.last_event = Some(event);
        self.active_view.draw_ui(&mut event_publisher, ctx);
    }
}

impl SecCardGameApp {

    fn handle_app_event(&mut self) {
        if let Some(app_event) = &self.last_event {
            match app_event {
                AppEvent::StartGame(data) => {
                    let deck = create_deck(&data.deck_composition, data.grace_rounds, &self.config);
                    self.active_view = Self::create_game_view_state(deck);
                }
            }
            self.last_event = None;
        };
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
