use game_lib::world::game::Game;
use game_setup::config::config::Config;
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

enum AppState {
    Init(),
    GameView(GameViewState)
}

pub(crate) struct SecCardGameApp {
    state: AppState,
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
