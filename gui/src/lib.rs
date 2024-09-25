use game_lib::world::game::Game;
use std::cell::RefCell;
use std::rc::Rc;
use crate::actions::command::Command;

mod app;
mod card_view_model;
mod card_window;
pub mod start;
mod actions;
mod components;
mod side_panel;

pub(crate) type CommandToExecute = Rc<RefCell<Option<Command>>>;

pub struct SecCardGameApp {
    game: Game,
    input: Input,
    command: CommandToExecute,
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
