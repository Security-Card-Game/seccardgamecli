use crate::game_view::actions::command::Command;
use crate::game_view::actions::command_handler::CommandHandler;
use crate::game_view::card_window::card_view_model::CardContent;
use crate::game_view::card_window::card_window::display_card;
use crate::{AppEvent, GameGoals, ViewState};
use egui::{Context, Ui};
use game_lib::cards::game_variants::scenario::Scenario;
use game_lib::world::board::Board;
use game_lib::world::deck::CardRc;
use game_lib::world::game::{Game, GameStatus};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub(super) type CommandToExecute = Option<Command>;

pub(super) enum Message {
    Failure(String),
    Warning(String),
    Success(String),
    None,
}

pub(super) struct Input {
    pub(super) next_res: String,
    pub(super) pay_res: String,
    pub(super) inc_reputation: String,
    pub(super) dec_reputation: String,
    pub(super) message: Message,
    pub(super) multiplier: String,
}

pub(crate) struct GameViewState {
    pub(super) game: Game,
    pub(super) input: Input,
    pub(super) command: CommandToExecute,
    pub(super) game_goals: GameGoals,
    pub(super) scenario: Option<Rc<Scenario>>,
}

impl ViewState for GameViewState {
    fn draw_ui(&mut self, _app_event_callback: &mut dyn FnMut(AppEvent), ctx: &Context) {
        self.process_command();
        self.create_side_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.update_cards(ctx, ui);
        });
    }
}

impl GameViewState {
    pub fn new(game: Game, game_goals: GameGoals, scenario: Option<Rc<Scenario>>) -> Self {
        let initial_gain = game.resource_gain.value().clone();
        let initial_multiplier = game.fix_multiplier.value().clone();
        GameViewState {
            game,
            input: Input {
                next_res: initial_gain.to_string(),
                dec_reputation: "0".to_string(),
                inc_reputation: "0".to_string(),
                pay_res: "0".to_string(),
                message: Message::None,
                multiplier: initial_multiplier.to_string(),
            },
            command: None,
            game_goals,
            scenario,
        }
    }
    fn update_cards(&mut self, ctx: &Context, ui: &mut Ui) {
        match &self.game.status {
            GameStatus::Start(board)
            | GameStatus::InProgress(board)
            | GameStatus::Finished(board) => {
                let cloned_board = board.clone();
                self.display_cards(&cloned_board, ctx, ui);
            }
        }
    }

    fn display_cards(&mut self, board: &Board, ctx: &Context, ui: &mut Ui) {
        for card in <HashMap<Uuid, CardRc> as Clone>::clone(&board.open_cards).into_iter() {
            let card_to_display = CardContent::from_card(
                &card.0,
                card.1.clone(),
                self.game.is_card_activated(&card.0),
                self.game.fix_multiplier.clone(),
            );
            let mut set_command = |cmd| self.command = Some(cmd);
            display_card(&card_to_display, &mut set_command, ctx, ui);
        }
    }
}
