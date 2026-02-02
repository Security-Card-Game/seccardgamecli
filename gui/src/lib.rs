use crate::game_view::state::GameViewState;
use egui::Context;
use game_lib::world::deck::DeckComposition;
use game_lib::world::game::GameInitSettings;
use game_setup::config::config::Config;

mod app;
mod components;
mod game_view;
mod init_view;
pub mod start;

trait ViewState {
    fn draw_ui(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ctx: &Context);
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StartGameData {
    pub deck_composition: DeckComposition,
    pub grace_rounds: u8,
    pub game_init_settings: GameInitSettings,
}

impl AppEvent {
    pub fn start_game(start_game_data: StartGameData) -> Self {
        AppEvent::StartGame(start_game_data)
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum AppEvent {
    StartGame(StartGameData),
}

pub(crate) struct SecCardGameApp {
    active_view: Box<dyn ViewState>,
    last_event: Option<AppEvent>,
    config: Config,
}
