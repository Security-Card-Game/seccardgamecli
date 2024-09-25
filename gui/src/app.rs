use egui::{Context, Ui};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

use game_lib::world::board::Board;
use game_lib::world::deck::{CardRc, Deck};
use game_lib::world::game::{Game, GameStatus};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

use super::{Input, Message, SecCardGameApp};
use crate::actions::command_handler::CommandHandler;
use crate::card_window::card_view_model::CardContent;
use crate::card_window::card_window::display_card;

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
            command: Rc::new(RefCell::new(None))
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

    fn display_cards(&mut self,
                     board: &Board,
                     ctx: &Context,
                     ui: &mut Ui
    ) {
        for card in <HashMap<Uuid, CardRc> as Clone>::clone(&board.open_cards).into_iter() {
            let card_to_display = CardContent::from_card(
                &card.0,
                card.1.clone(),
                self.game.is_card_activated(&card.0),
                self.game.fix_multiplier.clone(),
            );
            display_card(
                &card_to_display,
                self.command.clone(),
                ctx,
                ui,
            );
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
        self.process_command();

        Self::create_menu_bar(ctx);

        self.create_side_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.update_cards(ctx, ui);
        });
    }
}
