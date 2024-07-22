use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::cards::properties::duration::Duration;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::world::actions::action_error::{ActionError, ActionResult};
use crate::world::actions::action_error::ActionError::WrongCardType;
use crate::world::board::Board;
use crate::world::deck::CardRc;

/*
Decreased the duration of all AttackCards with a limited duration. Removes the cards from the board
if the hit zero duration.
*/
pub fn update_attack_cards(board: Board) -> Board {
    let mut open_cards = HashMap::new();
    for (key, card) in board.open_cards.iter() {
        let card_to_insert = match &**card {
            Card::Attack(ac) => update_attack_card(card, ac),
            Card::Event(_) | Card::Oopsie(_) | Card::Lucky(_) => Some(card.clone()),
        };

        match card_to_insert {
            None => {}
            Some(c) => {
                open_cards.insert(key.clone(), c);
            }
        }
    }

    Board {
        open_cards: open_cards.clone(),
        ..board
    }
}

fn update_attack_card(card: &CardRc, ac: &AttackCard) -> Option<Rc<Card>> {
    let new_duration = ac.duration.decrease();
    if let Some(value) = new_duration.value() {
        if value.clone() == 0 {
            return None
        }
    };

    match new_duration {
        Duration::Rounds(_) => {
            let attack_card = AttackCard {
                title: ac.title.clone(),
                description: ac.description.clone(),
                effect: ac.effect.clone(),
                duration: new_duration,
            };
            Some(Rc::new(Card::from(attack_card)))
        },
        Duration::UntilClosed => Some(card.clone()),
        Duration::None => None,
    }
}

/// Try to close an attack card manually by its ID.
/// - If the card attack card has not zero duration, returns 'AttackForceClosed'.
/// - If the card is not found, return an `InvalidState` error.
/// - If it's the wrong type of card, return a `WrongCardType` error.
/// - Otherwise, close the attack card.
pub fn manually_close_attack_card(board: Board, card_id: &Uuid) -> ActionResult<Board> {
    if let Some(card) = board.open_cards.get(card_id) {
        match &**card {
            Card::Oopsie(_)
            | Card::Lucky(_)
            | Card::Event(_) => Err(WrongCardType(board.clone())),
            Card::Attack(ac) => close_attack_card(&ac.duration.clone(), board, card_id),
        }
    } else {
        Err(ActionError::InvalidState(board.clone()))
    }
}

fn close_attack_card(duration: &Duration, board: Board, id: &Uuid) -> ActionResult<Board> {
    let open_cards = &mut board.open_cards.clone();
    open_cards.remove(id);

    let updated_board = Board {
        open_cards: open_cards.clone(),
        ..board
    };

    match duration {
        Duration::Rounds(r) =>  if r == &0usize { Ok(updated_board) } else { Err(ActionError::AttackForceClosed(updated_board))}
        Duration::UntilClosed => Err(ActionError::AttackForceClosed(updated_board)),
        Duration::None => Ok(updated_board)
    }
}