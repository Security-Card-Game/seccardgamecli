use crate::game_view::state::GameViewState;
use egui::Ui;
use game_lib::cards::game_variants::scenario::Scenario;
use game_lib::world::deck::DeckComposition;
use game_lib::world::game::GameInitSettings;
use game_setup::config::config::Config;
use std::rc::Rc;

mod app;
mod components;
mod game_view;
mod init_view;
pub mod start;

/// A 'screen' of the game. E.g. setup screen and play screen.
trait ViewState {
   
    /// Mimics [`eframe::App::logic`] and is called from that function.
    /// Cannot call UI and is not allowed to draw Ui elements.
    /// Is also called if Ui is hidden.
    fn logic(&mut self)  { 
        // nothing to do 
    }

    /// Mimics [`eframe::App::ui`] and is called from that function.
    /// Should not contain logic but draws the Ui.
    fn ui(&mut self, app_event_callback: &mut dyn FnMut(AppEvent), ui: &mut Ui);
}

#[derive(Debug, Clone)]
pub(crate) struct StartGameData {
    pub deck_composition: DeckComposition,
    pub grace_rounds: u8,
    pub game_init_settings: GameInitSettings,
    pub game_goals: GameGoals,
    pub scenario: Option<Rc<Scenario>>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GameGoals {
    pub min_resources: usize,
    pub min_reputation: u8,
}

impl Default for GameGoals {
    fn default() -> Self {
        GameGoals {
            min_resources: 0,
            min_reputation: 0,
        }
    }
}

impl AppEvent {
    pub fn start_game(start_game_data: StartGameData) -> Self {
        AppEvent::StartGame(start_game_data)
    }

    pub fn new_game() -> Self {
        AppEvent::NewGame
    }
}
#[derive(Debug, Clone)]
pub(crate) enum AppEvent {
    StartGame(StartGameData),
    NewGame,
}

pub(crate) struct SecCardGameApp {
    active_view: Box<dyn ViewState>,
    last_event: Option<AppEvent>,
    config: Config,
}
