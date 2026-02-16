use eframe::epaint::Color32;
use egui::{RichText, Ui};
use crate::game_view::state::{GameViewState, Message};

impl GameViewState {
    pub(crate) fn game_status_display(&mut self, ui: &mut Ui) {
        self.display_cards_remaining(ui);
        ui.add_space(20.0);
        self.display_message(ui);
    }

    fn display_cards_remaining(&mut self, ui: &mut Ui) {
        let card_count = &self.game.get_card_count();
        ui.label(format!(
            "Cards {}/{}",
            card_count.played_cards, card_count.total_cards
        ));
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
