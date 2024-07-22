use uuid::Uuid;

use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::ActionError::{InvalidState, WrongCardType};
use crate::world::actions::action_error::ActionResult;
use crate::world::board::Board;

pub fn activate_lucky_card(board: Board, id: &Uuid) -> ActionResult<Board> {
    check_id_and_perform(board, id, add_lucky_card)
}

pub fn deactivate_lucky_card(board: Board, id: &Uuid) -> ActionResult<Board> {
    check_id_and_perform(board, id, remove_lucky_card)
}


fn check_id_and_perform<F>(board: Board, id: &Uuid, func: F) -> ActionResult<Board>
where F: FnOnce(Board, &Uuid) -> Board {
    if let Some(card) = board.open_cards.get(id) {
        match  &**card {
            Card::Event(_)
            | Card::Attack(_)
            | Card::Oopsie(_) => Err(WrongCardType(board)),
            Card::Lucky(_) => Ok(func(board, id)),
        }
    } else {
        Err(InvalidState(board))
    }
}

fn add_lucky_card(board: Board, id: &Uuid) -> Board {
    let active_cards = &mut board.cards_to_use.clone();
    active_cards.insert(id.clone());
    Board {
        cards_to_use: active_cards.clone(),
        ..board
    }
}

fn remove_lucky_card(board: Board, id: &Uuid) -> Board {
    let active_cards = &mut board.cards_to_use.clone();
    active_cards.remove(id);
    Board {
        cards_to_use: active_cards.clone(),
        ..board
    }
}

