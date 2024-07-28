use std::collections::HashMap;

use egui::{Context, Ui};
use uuid::Uuid;

use game_lib::world::board::{Board};
use game_lib::world::deck::{CardRc, Deck};
use game_lib::world::game::{GameActionResult, Game, GameStatus};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

use crate::card_view_model::{CardContent, CardMarker};
use crate::card_window::display_card;

use super::{Input, Message, SecCardGameApp};

impl SecCardGameApp {
    fn init(deck: Deck) -> Self {
        let game = Game::create(deck, Resources::new(5), ResourceFixMultiplier::default());
        let initial_gain = game.resource_gain.value().clone();
        let initial_multiplier = game.fix_multiplier.value().clone();
        Self {
            game,
            input: Input {
                next_res: initial_gain.to_string(),
                pay_res: "0".to_string(),
                message: Message::None,
                multiplier: initial_multiplier.to_string(),
            },
        }
    }

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
        let mut ids_to_remove = vec![];
        for card in <HashMap<Uuid, CardRc> as Clone>::clone(&board.open_cards).into_iter() {
            let card_to_display = CardContent::from_card(
                &card.0,
                card.1.clone(),
                self.game.is_card_activated(&card.0),
                self.game.fix_multiplier.clone(),
            );
            display_card(
                &card_to_display,
                |id| ids_to_remove.push(id),
                |id, marker| match marker {
                    CardMarker::MarkedForUse => {
                        self.game = self.game.activate_lucky_card(&id);
                    }
                    CardMarker::None => self.game = self.game.deactivate_lucky_card(&id),
                },
                ctx,
                ui,
            );
        }

        // this handles the callback of the card to the board when the card is closed
        let mut new_turn: Option<Game> = None;
        for id in &ids_to_remove {
            let new_game_state = self.game.close_card(id);
            match &new_game_state.action_status {
                    GameActionResult::OopsieFixed(res) => {
                        self.input.message =
                            Message::Success(format!("Fixed for {} resources.", res.value()));
                    }
                    GameActionResult::FixFailed(res) => {
                        self.input.message = Message::Failure(format!(
                            "Fix failed! It would have needed {} resources.",
                            res.value()
                        ));
                    }
                    GameActionResult::AttackForceClosed => {
                        self.input.message =
                            Message::Warning("Attack forced to be over".to_string());
                    }
                GameActionResult::Payed => {}
                GameActionResult::NotEnoughResources => {}
                GameActionResult::NothingPayed => {}
                GameActionResult::InvalidAction => {
                    self.input.message =
                        Message::Failure("Invalid Action!".to_string())
                }
                GameActionResult::Success => {
                    self.input.message = Message::None
                }
            }

            new_turn = Some(new_game_state);
        }

        match new_turn {
            None => {}
            Some(g) => {
                self.game = g;
            }
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
    pub fn new(_cc: &eframe::CreationContext<'_>, deck: Deck) -> Self {
        SecCardGameApp::init(deck)
    }
}

impl eframe::App for SecCardGameApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Self::create_menu_bar(ctx);

        self.create_control_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.update_cards(ctx, ui);
        });
    }
}
