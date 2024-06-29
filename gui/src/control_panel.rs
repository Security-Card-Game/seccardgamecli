use eframe::epaint::Color32;
use egui::{Context, RichText, Ui};

use game_lib::cards::properties::fix_modifier::FixModifier;
use game_lib::world::board::CurrentBoard;
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

                self.game_status_display(ui);
            });
    }

}