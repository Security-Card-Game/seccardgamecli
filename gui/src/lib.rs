use egui::Context;
use crate::game_view::state::GameViewState;
use game_lib::world::deck::DeckComposition;
use game_setup::config::config::Config;

mod app;
pub mod start;
mod components;
mod init_view;
mod game_view;


trait ViewState {
    fn draw_ui(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ctx: &Context);
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StartGameData {
    deck_composition: DeckComposition,
    grace_rounds: u8,
}

impl AppEvent  {
    pub fn start_game(deck_composition: DeckComposition, grace_rounds: u8) -> Self {
        AppEvent::StartGame(StartGameData { deck_composition, grace_rounds })
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum AppEvent {
    StartGame(StartGameData),
}

pub(crate) struct SecCardGameApp {
    active_view: Box<dyn ViewState>,
    last_event: Option<AppEvent>,
    config: Config
}

