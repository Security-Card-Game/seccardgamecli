#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::Vec2;

use game_lib::cards::world::deck::Deck;

use crate::SecCardGameApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
pub fn run(deck: Deck) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_close_button(true)
            .with_resizable(true)
            .with_minimize_button(true)
            .with_inner_size(Vec2::new(1000.0, 600.0))
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "seccard game",
        native_options,
        Box::new(|cc| Box::new(SecCardGameApp::new(cc, deck))),
    )
}
