use game_lib::world::deck::DeckComposition;
use game_lib::world::game::Game;
use game_setup::config::config::Config;
use init_view::init_view::LabelValue;
use crate::actions::command::Command;

mod app;
mod card_window;
pub mod start;
mod actions;
mod components;
mod side_panel;
mod init_view;

pub(crate) type CommandToExecute = Option<Command>;

pub(crate) struct GameViewState {
    game: Game,
    input: Input,
    command: CommandToExecute,
}
pub(crate) struct InitViewState {
    event_card_count: LabelValue,
    attack_card_count: LabelValue,
    oopsie_card_count: LabelValue,
    lucky_card_count: LabelValue,
    evaluation_card_count: LabelValue,
    grace_rounds: LabelValue,
}

enum AppState {
    Init(InitViewState),
    GameView(GameViewState)
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
    state: AppState,
    last_event: Option<AppEvent>,
    config: Config
}

enum Message {
    Success(String),
    Failure(String),
    Warning(String),
    None,
}

struct Input {
    next_res: String,
    pay_res: String,
    inc_reputation: String,
    dec_reputation: String,
    message: Message,
    multiplier: String,
}
