use crate::{Message, SecCardGameApp};
use eframe::epaint::Color32;
use egui::{RichText, Ui};
use game_lib::world::game::GameStatus;

impl SecCardGameApp {
    pub(crate) fn game_status_display(&mut self, ui: &mut Ui) {
        self.display_cards_remaining(ui);
        ui.add_space(20.0);
        self.display_message(ui);
        ui.add_space(20.0);
        self.display_incidents(ui);
    }

    fn display_cards_remaining(&mut self, ui: &mut Ui) {
        let card_count = &self.game.get_card_count();
        ui.label(format!(
            "Cards {}/{}",
            card_count.played_cards, card_count.total_cards
        ));
    }

    fn display_incidents(&mut self, ui: &mut Ui) {
        let incidents = match &self.game.status {
            GameStatus::InProgress(board) => board.active_incidents.clone(),
            _ => Vec::new(),
        };

        if incidents.is_empty() {
            create_message(&"No incidents!".to_string(), Color32::GREEN, ui);
            return;
        }
        create_message(&format!("Incidents ({}):", incidents.len()), Color32::RED, ui);
        for incident in incidents {
            ui.label(format!(
                "{} -> {}",
                incident.attack_title, incident.oopsie_title
            ));
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
