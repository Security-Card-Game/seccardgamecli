/// # Card Command handling
/// Commands which can be triggered by a card should be handled here.
use uuid::Uuid;
use crate::SecCardGameApp;

impl SecCardGameApp {
    pub(super) fn handle_card_closed(&mut self, card_id: Uuid) {
        let new_game_state = self.game.close_card(&card_id);
        self.game = new_game_state;
    }

    pub(super) fn handle_activate_card(&mut self, card_id: Uuid) {
        let new_game_state = self.game.activate_lucky_card(&card_id);
        self.game = new_game_state;
    }

    pub(super) fn handle_deactivate_card(&mut self, card_id: Uuid) {
        let new_game_state = self.game.deactivate_lucky_card(&card_id);
        self.game = new_game_state;
    }
}
