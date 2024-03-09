use std::collections::HashMap;

use egui::{Color32, Context, RichText, Ui};
use rand::Rng;
use uuid::Uuid;

use game_lib::world::current_turn::CurrentBoard;
use game_lib::world::deck::{CardRc, Deck};
use game_lib::world::game::Game;
use game_lib::world::resources::Resources;

use crate::card::CardContent;
use crate::card_window::display_card;

pub struct SecCardGameApp {
    game: Game,
    resources_per_round: usize,
    input: Input,
}

struct Input {
    next_res: String,
    pay_res: String,
    dice: DiceRange,
    error: Option<String>,
    dice_result: Option<usize>,
}

struct DiceRange {
    min: String,
    max: String,
}
impl SecCardGameApp {
    fn init(turn: CurrentBoard) -> Self {
        let game = Game::Start(turn);
        Self {
            game,
            resources_per_round: 5,
            input: Input {
                next_res: "5".to_owned(),
                pay_res: "0".to_owned(),
                dice: DiceRange {
                    min: "0".to_string(),
                    max: "0".to_string(),
                },
                dice_result: None,
                error: None,
            },
        }
    }

    fn update_cards(&mut self, ctx: &Context, ui: &mut Ui) {
        match &self.game {
            Game::Start(board) | Game::InProgress(board) => {
                let cloned_board = board.clone();
                self.display_cards(&cloned_board, ctx, ui);
            }
            Game::Finished => {}
        }
    }

    fn display_cards(&mut self, board: &CurrentBoard, ctx: &Context, ui: &mut Ui) {
        let mut ids_to_remove = vec![];
        for card in <HashMap<Uuid, CardRc> as Clone>::clone(&board.open_cards).into_iter() {
            let card_to_display = CardContent::from_card(&card.0, card.1.clone());
            display_card(&card_to_display, |id| ids_to_remove.push(id), ctx, ui);
        }
        // this handles the callback of the card to the board when the card is closed
        let mut new_turn: Option<CurrentBoard> = None;
        for id in &ids_to_remove {
            new_turn = Some(board.close_card(id));
        }

        match new_turn {
            None => {}
            Some(b) => {
                self.game = Game::InProgress(b);
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

                self.dice_control(ui);

                ui.add_space(10.0);
                match &self.game {
                    Game::Finished => {}
                    Game::Start(board) | Game::InProgress(board) => {
                        ui.label(format!(
                            "Cards {}/{}",
                            board.deck.played_cards, board.deck.total
                        ));
                    }
                }
                ui.add_space(20.0);
                match &self.input.error {
                    None => {}
                    Some(e) => {
                        ui.label(RichText::new(e).color(Color32::RED));
                    }
                }
            });
    }

    fn dice_control(&mut self, ui: &mut Ui) {
        ui.label("Dice");
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Min:\t");
            ui.text_edit_singleline(&mut self.input.dice.min)
        });
        ui.horizontal(|ui| {
            ui.label("Max:\t");
            ui.text_edit_singleline(&mut self.input.dice.max)
        });
        if ui.button("Roll").clicked() {
            let min: usize = self.input.dice.min.parse().unwrap_or_else(|_| 0);
            let max: usize = self.input.dice.max.parse().unwrap_or_else(|_| 0);
            let mut rng = rand::thread_rng();
            let value = if min > max {
                rng.gen_range(max..min)
            } else if min == max {
                min
            } else {
                rng.gen_range(min..max)
            };
            self.input.dice_result = Some(value);
        }
        ui.add_space(5.0);
        match self.input.dice_result {
            None => ui.label(""),
            Some(value) => ui.label(format!("You rolled {}", value)),
        };
    }

    fn resource_control(&mut self, ui: &mut Ui) {
        ui.label("Resources");
        ui.add_space(5.0);
        match &self.game {
            Game::Start(board) | Game::InProgress(board) => {
                let cloned_board = board.clone();
                let available = RichText::new(format!(
                    "{} available",
                    cloned_board.current_resources.value()
                ))
                .strong();
                ui.label(available);

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input.pay_res);
                    ui.add_space(5.0);

                    if ui.button("Pay").clicked() {
                        let to_pay = self.input.pay_res.parse().unwrap_or_else(|_| 0);

                        if &to_pay > cloned_board.current_resources.value() {
                            self.input.error = Some("No money!".to_string())
                        } else {
                            self.input.pay_res = "0".to_string();
                            self.input.error = None;
                            self.game = Game::InProgress(
                                cloned_board.pay_resources(&Resources::new(to_pay)),
                            );
                        }
                    };
                });
            }
            Game::Finished => {}
        }
    }

    fn next_round_controls(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.label("Next round");
        ui.add_space(5.0);
        ui.label("Gain resources ");
        let res = ui.add(egui::TextEdit::singleline(&mut self.input.next_res).interactive(true));
        if res.lost_focus() {
            self.resources_per_round = self
                .input
                .next_res
                .parse()
                .unwrap_or_else(|_| self.resources_per_round);
        }

        ui.add_space(5.0);
        match &self.game {
            Game::Finished => {
                ui.label("Game ended");
            }
            Game::Start(board) | Game::InProgress(board) => {
                if board.turns_remaining > 0 && ui.button("Draw card").clicked() {
                    self.input.dice_result = None;
                    self.input.error = None;
                    self.game = Game::InProgress(
                        board.next_round(Resources::new(self.resources_per_round)),
                    );
                }
            }
        };
    }
}

impl SecCardGameApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, deck: Deck) -> Self {
        let turn = CurrentBoard::init(deck, Resources::new(0));
        SecCardGameApp::init(turn)
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
