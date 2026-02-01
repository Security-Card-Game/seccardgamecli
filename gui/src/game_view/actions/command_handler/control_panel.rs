/// # Control Panel handling
/// Commands which can be triggered by a the control panel should be handled here.
use crate::{GameViewState, Message};
use game_lib::world::reputation::Reputation;
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;
use game_lib::world::resources::Resources;


impl GameViewState {
    pub(crate) fn handle_pay_resources(&mut self, res: usize) {
        self.game = self.game.pay_resources(&Resources::new(res));
    }

    pub(crate) fn handle_set_multiplier(&mut self, m: isize) {
        if m <= 0 {
            self.input.message = Message::Failure("Invalid Action, must be > 0!".to_string());
        }
        self.game = self
            .game
            .set_fix_multiplier(ResourceFixMultiplier::new(m.unsigned_abs()));
    }

    pub(crate) fn handle_set_resource_gain(&mut self, res: usize) {
        self.game = self.game.set_resource_gain(Resources::new(res));
        self.input.next_res = res.to_string();
    }

    pub(crate) fn handle_increase_reputation(&mut self, rep: u8) {
        self.game = self.game.increase_reputation(&Reputation::new(rep));
    }

    pub(crate) fn handle_decrease_reputation(&mut self, rep: u8) {
        self.game = self.game.decrease_reputation(&Reputation::new(rep));
    }

}
