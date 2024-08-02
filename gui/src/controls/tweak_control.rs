use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use egui::Ui;

use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;

use crate::{Message, SecCardGameApp};

impl SecCardGameApp {
    pub(crate) fn tweak_control(&mut self, ui: &mut Ui) {
        ui.label("Tweaks");
        ui.add_space(5.0);

        ui.label("Multiply all fix costs by:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.input.multiplier);
            ui.add_space(5.0);

            if ui.button("Set Multiplier").clicked() {
                let new_multiplier: isize = self.input.multiplier.parse().unwrap_or_else(|_| 1);
                if new_multiplier <= 0 {
                   self.input.message = Message::Failure("Invalid Action, must be > 0!".to_string());
                    return;
                }
                self.input.message = Message::None;
                self.game = self
                    .game
                    .set_fix_multiplier(ResourceFixMultiplier::new(new_multiplier.unsigned_abs()));
            }

            ui.label("Round Duration");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input.round_duration);
                ui.add_space(5.0);
                ui.label("minutes");
                if ui.button("Set").clicked() {
                    let duration: i32 = self.input.round_duration.parse().unwrap_or_else(|_| 1);
                    if duration <= 1 {
                        self.input.message = Message::Failure("Invalid Action, must be >= 1!".to_string());
                        return;
                    }
                    self.timer.duration = duration.unsigned_abs() as usize;
                }
            });


            if ui.button("Start Timer").clicked() {
                let new_multiplier: isize = self.input.multiplier.parse().unwrap_or_else(|_| 1);
                if new_multiplier <= 0 {
                    self.input.message = Message::Failure("Invalid Action, must be > 0!".to_string());
                    return;
                }
                self.input.message = Message::None;
                self.game = self
                    .game
                    .set_fix_multiplier(ResourceFixMultiplier::new(new_multiplier.unsigned_abs()));
                let countdown_seconds = Arc::new(Mutex::new(self.timer.duration * 60));
                let seconds_clone = Arc::clone(&countdown_seconds);
                tokio::spawn(start_countdown(seconds_clone));

                let remaining_seconds = countdown_seconds.lock().unwrap();
                ui.label(format!("Seconds remaining {}", *remaining_seconds));
            };
        });
    }
}

async fn start_countdown(seconds: Arc<Mutex<usize>>) {
    let mut remaining = seconds.lock().unwrap();
    while *remaining > 0 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        *remaining -= 1;
    }
}
