#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::SecCardGameApp;
use game_lib::world::deck::Deck;
use game_setup::config::config::Config;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
pub fn run(predefined_deck: Option<Deck>, config: Config) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_close_button(true)
            .with_resizable(true)
            .with_minimize_button(true),
        ..Default::default()
    };
    if let Some(deck) = predefined_deck {
        eframe::run_native(
            "seccard game",
            native_options,
            Box::new(|cc| Ok(Box::new(SecCardGameApp::new_with_deck(cc, deck, config)))),
        )
    } else {
        eframe::run_native(
            "seccard game",
            native_options,
            Box::new(|cc| Ok(Box::new(SecCardGameApp::new(cc, config)))),
        )
    }
}
