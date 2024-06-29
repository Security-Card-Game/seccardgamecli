use eframe::epaint::Color32;
use egui::{RichText, Ui};

use game_lib::world::game::GameStatus;

use crate::{Message, SecCardGameApp};

impl SecCardGameApp {
    pub(crate) fn game_status_display(&mut self, ui: &mut Ui) {
        self.display_cards_remaining(ui);
        ui.add_space(20.0);
        self.display_message(ui);

    }

    fn display_cards_remaining(&mut self, ui: &mut Ui) {
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
    }

    fn display_message(&mut self, ui: &mut Ui) {
        match &self.input.message {
            Message::Success(m) => create_message(m, Color32::GREEN, ui),
            Message::Failure(m) => create_message(m, Color32::RED, ui),
            Message::Warning(m) => create_message(m, Color32::GOLD, ui),
            Message::None => {}
        }
    }
}

fn create_message(message: &String, color: Color32, ui: &mut Ui) {
    ui.label(RichText::new(message).color(color));
}
