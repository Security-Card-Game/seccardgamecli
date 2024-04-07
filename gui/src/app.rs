use std::collections::HashMap;

use egui::{Color32, Context, RichText, Ui};
use uuid::Uuid;

use game_lib::cards::properties::fix_modifier::FixModifier;
use game_lib::world::board::CurrentBoard;
use game_lib::world::deck::{CardRc, Deck};
use game_lib::world::game::{ActionResult, Game, GameStatus, Payment};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

use crate::card_view_model::{CardContent, CardMarker};
use crate::card_window::display_card;

pub struct SecCardGameApp {
    game: Game,
    input: Input,
}

enum Message {
    Success(String),
    Failure(String),
    Warning(String),
    None,
}

struct Input {
    next_res: String,
    pay_res: String,
    message: Message,
    multiplier: String,
}

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

    fn display_cards(&mut self, board: &CurrentBoard, ctx: &Context, ui: &mut Ui) {
        let mut ids_to_remove = vec![];
        for card in <HashMap<Uuid, CardRc> as Clone>::clone(&board.open_cards).into_iter() {
            let card_to_display = CardContent::from_card(
                &card.0,
                card.1.clone(),
                self.game.active_cards.contains_key(&card.0),
                self.game.fix_multiplier.clone(),
            );
            display_card(
                &card_to_display,
                |id| ids_to_remove.push(id),
                |id, marker| match marker {
                    CardMarker::MarkedForUse => {
                        self.game = self.game.activate_card(&id);
                    }
                    CardMarker::None => self.game = self.game.deactivate_card(&id),
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
                None => {
                    self.input.message = Message::None;
                }
                Some(res) => match res {
                    ActionResult::OopsieFixed(res) => {
                        self.input.message =
                            Message::Success(format!("Fixed for {} resources.", res.value()));
                    }
                    ActionResult::FixFailed(res) => {
                        self.input.message = Message::Failure(format!(
                            "Fix failed! It would have needed {} resources.",
                            res.value()
                        ));
                    }
                    ActionResult::AttackForceClosed => {
                        self.input.message =
                            Message::Warning("Attack forced to be over".to_string());
                    }
                },
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

    fn create_control_panel(&mut self, ctx: &Context) {
        egui::SidePanel::left("control_panel")
            .resizable(false)
            .max_width(100.0)
            .show(ctx, |ui| {
                self.next_round_controls(ui);

                ui.add_space(15.0);

                self.resource_control(ui);

                ui.add_space(15.0);

                self.tweak_control(ui);

                ui.add_space(10.0);
                match &self.game.status {
                    GameStatus::Start(board)
                    | GameStatus::InProgress(board)
                    | GameStatus::Finished(board) => {
                        ui.label(format!(
                            "Cards {}/{}",
                            board.deck.played_cards, board.deck.total
                        ));
                    }
                }

                ui.add_space(20.0);
                match &self.input.message {
                    Message::Success(m) => Self::show_message(m, Color32::GREEN, ui),
                    Message::Failure(m) => Self::show_message(m, Color32::RED, ui),
                    Message::Warning(m) => Self::show_message(m, Color32::GOLD, ui),
                    Message::None => {}
                }
            });
    }

    fn show_message(message: &String, color: Color32, ui: &mut Ui) {
        ui.label(RichText::new(message).color(color));
    }

    fn tweak_control(&mut self, ui: &mut Ui) {
        ui.label("Tweaks");
        ui.add_space(5.0);

        ui.label("Mutliply all fix costs by:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input.multiplier);
            ui.add_space(5.0);

            if ui.button("Set Multiplier").clicked() {
                let new_multiplier = self.input.multiplier.parse().unwrap_or_else(|_| 1);
                self.game = self
                    .game
                    .set_fix_multiplier(ResourceFixMultiplier::new(new_multiplier));
            }
        });
    }

    fn resource_control(&mut self, ui: &mut Ui) {
        ui.label("Resources");
        ui.add_space(5.0);
        match &self.game.status {
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                let cloned_board = board.clone();
                let available = RichText::new(format!(
                    "{} available",
                    cloned_board.current_resources.value()
                ))
                .strong();
                ui.label(available);

                let modifier = match &self.game.get_current_fix_modifier() {
                    None => "No cost modifier active!".to_string(),
                    Some(m) => match m {
                        FixModifier::Increase(r) => {
                            format!("Next fix is increased by: {}", r.value()).to_string()
                        }
                        FixModifier::Decrease(r) => {
                            format!("Next fix is decreased by: {}", r.value()).to_string()
                        }
                    },
                };
                ui.label(modifier);

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input.pay_res);
                    ui.add_space(5.0);

                    if ui.button("Pay").clicked() {
                        let to_pay = self.input.pay_res.parse().unwrap_or_else(|_| 0);
                        self.game = match self.game.pay_resources(&Resources::new(to_pay)) {
                            Payment::Payed(g) | Payment::NothingPayed(g) => {
                                self.input.pay_res = "0".to_string();
                                self.input.message = Message::None;
                                g
                            }
                            Payment::NotEnoughResources(g) => {
                                self.input.message =
                                    Message::Warning("Not enough resources!".to_string());
                                g
                            }
                        };
                    };
                });
            }
            GameStatus::Finished(_) => {}
        }
    }

    fn next_round_controls(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.label("Next round");
        ui.add_space(5.0);
        ui.label("Gain resources ");
        let res = ui.add(egui::TextEdit::singleline(&mut self.input.next_res).interactive(true));
        if res.lost_focus() {
            let new_gain = self.input.next_res.parse().unwrap_or(0usize);

            self.game = self.game.set_resource_gain(Resources::new(new_gain));
            self.input.next_res = self.game.resource_gain.value().to_string();
        }

        ui.add_space(5.0);
        match &self.game.status {
            GameStatus::Finished(_) => {
                ui.label("Game ended");
            }
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                if board.turns_remaining > 0 && ui.button("Draw card").clicked() {
                    self.input.message = Message::None;
                    self.game = self.game.next_round();
                }
            }
        };
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
