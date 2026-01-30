use egui::{Context, Ui};
use std::collections::HashMap;
use uuid::Uuid;

use super::{AppEvent, GameViewState, Input, Message, SecCardGameApp};
use crate::actions::command_handler::CommandHandler;
use crate::card_window::card_view_model::CardContent;
use crate::card_window::card_window::display_card;
use crate::init_view::init_view::InitViewState;
use crate::AppState::{GameView, Init};
use game_lib::world::board::Board;
use game_lib::world::deck::{CardRc, Deck};
use game_lib::world::game::{Game, GameStatus};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;
use game_setup::config::config::Config;
use game_setup::creation::create::create_deck;

impl SecCardGameApp {
    fn start_game(deck: Deck, config: Config) -> Self {
        let game_view = Self::create_game_view_state(deck);
        Self {
            state: GameView(game_view),
            last_event: None,
            config
        }
    }

    pub fn create_game_view_state(deck: Deck) -> GameViewState {
        let game = Game::create(deck, Resources::new(5), ResourceFixMultiplier::default());
        let initial_gain = game.resource_gain.value().clone();
        let initial_multiplier = game.fix_multiplier.value().clone();
        let game_view = GameViewState {
            game,
            input: Input {
                next_res: initial_gain.to_string(),
                dec_reputation: "0".to_string(),
                inc_reputation: "0".to_string(),
                pay_res: "0".to_string(),
                message: Message::None,
                multiplier: initial_multiplier.to_string(),
            },
            command: None,
        };
        game_view
    }

    fn init(config: Config) -> Self {
        Self { state: Init(InitViewState::new()), last_event: None, config }
    }
}
impl GameViewState {
    fn update_cards(&mut self, ctx: &Context, ui: &mut Ui) {
        match &self.game.status {
            GameStatus::Start(board)
            | GameStatus::InProgress(board)
            | GameStatus::Finished(board) => {
                let cloned_board = board.clone();
                self.display_cards(&cloned_board, ctx, ui);
            }
        }
    }

    fn display_cards(&mut self, board: &Board, ctx: &Context, ui: &mut Ui) {
        for card in <HashMap<Uuid, CardRc> as Clone>::clone(&board.open_cards).into_iter() {
            let card_to_display = CardContent::from_card(
                &card.0,
                card.1.clone(),
                self.game.is_card_activated(&card.0),
                self.game.fix_multiplier.clone(),
            );
            let mut set_command = |cmd| self.command = Some(cmd);
            display_card(&card_to_display, &mut set_command, ctx, ui);
        }
    }

}

impl SecCardGameApp {
    /// Called once before the first frame.
    pub fn new_with_deck(cc: &eframe::CreationContext<'_>, deck: Deck, config: Config) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.4);
        SecCardGameApp::start_game(deck, config)
    }

    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.4);
        SecCardGameApp::init(config)
    }
}

impl eframe::App for SecCardGameApp {


    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Self::create_menu_bar(ctx);
        if let Some(app_event) = &self.last_event {
                match app_event {
                    AppEvent::StartGame(data) => {
                        let deck = create_deck(&data.deck_composition, data.grace_rounds, &self.config);
                        self.state = GameView(Self::create_game_view_state(deck));
                    }
                }
                self.last_event= None;
        }

        match &mut self.state {
            Init(state) => {
                let mut set_event =  |event| self.last_event = Some(event);
                state.draw_ui(
                    &mut set_event,
                    ctx);
            }
            GameView(gv) => {
                gv.process_command();
                gv.create_side_panel(ctx);
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The central panel the region left after adding TopPanel's and SidePanel's
                    gv.update_cards(ctx, ui);
                });
            }
        }
    }
}

impl SecCardGameApp {
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
