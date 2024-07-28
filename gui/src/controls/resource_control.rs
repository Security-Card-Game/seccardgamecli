use egui::{RichText, Ui};

use game_lib::cards::properties::fix_modifier::FixModifier;
use game_lib::world::board::{Board};
use game_lib::world::game::{GameActionResult, GameStatus};
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;

use crate::{Message, SecCardGameApp};

impl SecCardGameApp {

    pub(crate) fn resource_control(&mut self, ui: &mut Ui) {
        ui.label("Resources");
        ui.add_space(5.0);
        match &self.game.status.clone() {
            GameStatus::Start(board) | GameStatus::InProgress(board) => {
                self.game_in_progress(ui, board);
            }
            GameStatus::Finished(board) => {
                game_ended(ui, board);
            }
        }
    }

    fn game_in_progress(&mut self, ui: &mut Ui, board: &Board) {
        let cloned_board = board.clone();
        let available = create_resource_label(&cloned_board, "available");
        ui.label(available);

        let modifier = create_fix_modifier_label(self.game.get_current_fix_modifier(), &self.game.fix_multiplier);
        ui.label(modifier);

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input.pay_res);
            ui.add_space(5.0);
            if ui.button("Pay").clicked() {
                let to_pay = self.input.pay_res.parse().unwrap_or_else(|_| 0);
                self.game = self.game.pay_resources(&Resources::new(to_pay));
                match self.game.action_status {
                    GameActionResult::Payed => {}
                    | GameActionResult::NothingPayed => {
                        self.input.pay_res = "0".to_string();
                        self.input.message = Message::None;
                    }
                        GameActionResult::NotEnoughResources => {
                        self.input.message =
                            Message::Warning("Not enough resources!".to_string());
                    }
                    _ => {}
                }
            };
        });
    }
}

fn game_ended(ui: &mut Ui, board: &Board) {
    let available = create_resource_label(board, "left");
    ui.label(available);
}


// helper function to create rich text for resource amount,
// will be used in match arms for GameStatus
fn create_resource_label(board: &Board, postfix: &str) -> RichText {
    let resource_str = format!(
        "{} {}", board.current_resources.value(), postfix
    );
    RichText::new(resource_str)
        .strong()
}

// function to create fix modifier label
fn create_fix_modifier_label(fix_modifier: Option<FixModifier>, multiplier: &ResourceFixMultiplier) -> String {
    match fix_modifier {
        None => "No cost modifier active!".to_string(),
        Some(m) => {
            match m {
                FixModifier::Increase(r) => format!("Next fix is increased by: {}", r * multiplier),
                FixModifier::Decrease(r) => format!("Next fix is decreased by: {}", r * multiplier),
            }
        }
    }
}
