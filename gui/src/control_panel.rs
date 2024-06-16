use eframe::epaint::Color32;
use egui::{Context, RichText, Ui};

use game_lib::cards::properties::fix_modifier::FixModifier;
use game_lib::world::game::{GameStatus, Payment};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

use super::{Message, SecCardGameApp};

impl SecCardGameApp {
    pub(crate) fn create_control_panel(&mut self, ctx: &Context) {
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
            GameStatus::Finished(board) => {
                let available = RichText::new(format!(
                    "{} left",
                    board.current_resources.value().clone()
                ))
                    .strong();
                ui.label(available);
            }
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