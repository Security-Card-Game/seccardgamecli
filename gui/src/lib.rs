use game_lib::world::game::Game;

mod app;
mod card_view_model;
mod card_window;
pub mod start;
mod control_panel;
mod controls;

pub struct SecCardGameApp {
    game: Game,
    input: Input,
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
    message: Message,
    multiplier: String,
}
