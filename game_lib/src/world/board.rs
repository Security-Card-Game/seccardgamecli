use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use uuid::Uuid;

use crate::cards::properties::duration::Duration;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::world::deck::{CardRc, Deck};
use crate::world::resources::Resources;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CardRcWithId {
    pub id: Uuid,
    pub card: CardRc,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub current_resources: Resources,
    pub(crate) drawn_card: Option<CardRcWithId>,
    pub open_cards: HashMap<Uuid, CardRc>,
    pub cards_to_use: HashSet<Uuid>,
    pub fix_modifier: Option<FixModifier>,
    pub turns_remaining: usize,
}

impl Board {
    pub fn init(deck: &Deck, start_resources: Resources) -> Self {
        Board {
            current_resources: start_resources,
            drawn_card: None,
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            fix_modifier: None,
            turns_remaining: deck.total,
        }
    }

    pub fn empty() -> Self {
        Board {
            current_resources: Resources::new(0),
            drawn_card: None,
            open_cards: HashMap::new(),
            cards_to_use: HashSet::new(),
            fix_modifier: None,
            turns_remaining: 0,
        }
    }
}

struct BoardAndDeck {
    board: Board,
    deck: Deck,
}

impl From<BoardAndDeck> for CurrentBoard {
    fn from(value: BoardAndDeck) -> Self {
        CurrentBoard {
            current_resources: value.board.current_resources,
            drawn_card: value.board.drawn_card,
            open_cards: value.board.open_cards,
            deck: value.deck,
            turns_remaining: value.board.turns_remaining,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentBoard {
    pub current_resources: Resources,
    pub(crate) drawn_card: Option<CardRcWithId>,
    pub open_cards: HashMap<Uuid, CardRc>,
    pub deck: Deck, // this should not be public, but is needed at the moment for game creation
    pub turns_remaining: usize,
}

impl CurrentBoard {
    pub fn init(deck: Deck, start_resources: Resources) -> Self {
        let total_rounds = &deck.total.clone();
        CurrentBoard {
            current_resources: start_resources,
            drawn_card: None,
            open_cards: HashMap::new(),
            deck,
            turns_remaining: *total_rounds,
        }
    }

    pub(crate) fn next_round(&self, new_resources: Resources) -> Self {
        let current_resources = &self.current_resources;
        let deck = &self.deck;
        let cards = &self.deck.remaining_cards;
        let open_cards = &mut update_open_cards(self.open_cards.clone());
        let (drawn_card, rest) = cards.split_at(1);
        let card_ref = Rc::new(drawn_card[0].clone());
        let card_id = Uuid::new_v4();

        open_cards.insert(card_id, card_ref.clone());

        CurrentBoard {
            current_resources: new_resources + current_resources.clone(),
            drawn_card: Some(CardRcWithId {
                id: card_id,
                card: card_ref,
            }),
            open_cards: open_cards.clone(),
            deck: deck.with_remaining_cards(rest),
            turns_remaining: self.turns_remaining - 1,
        }
    }

    pub(crate) fn close_card(&self, card_id: &Uuid) -> Self {
        let open_cards = &mut self.open_cards.clone();
        open_cards.remove(card_id);

        CurrentBoard {
            current_resources: self.current_resources.clone(),
            drawn_card: self.drawn_card.clone(),
            open_cards: open_cards.clone(),
            deck: self.deck.clone(),
            turns_remaining: self.turns_remaining,
        }
    }

    pub(crate) fn pay_resources(&self, resources: &Resources) -> Self {
        CurrentBoard {
            current_resources: self.current_resources.clone() - resources.clone(),
            drawn_card: self.drawn_card.clone(),
            open_cards: self.open_cards.clone(),
            deck: self.deck.clone(),
            turns_remaining: self.turns_remaining,
        }
    }
}

fn update_open_cards(input: HashMap<Uuid, CardRc>) -> HashMap<Uuid, CardRc> {
    let mut result = HashMap::new();
    for (key, card) in input.iter() {
        let card_to_insert = match &**card {
            Card::Attack(ac) => update_attack_card(card, ac),
            Card::Event(_) | Card::Oopsie(_) | Card::Lucky(_) => Some(card.clone()),
        };

        match card_to_insert {
            None => {}
            Some(c) => {
                result.insert(*key, c);
            }
        }
    }
    result
}

fn update_attack_card(card: &CardRc, ac: &AttackCard) -> Option<Rc<Card>> {
    let new_duration = ac.duration.decrease();
    match new_duration {
        Duration::Rounds(_) => {
            let updated_card = Card::Attack(AttackCard {
                title: ac.title.clone(),
                description: ac.description.clone(),
                effect: ac.effect.clone(),
                duration: new_duration,
            });
            Some(Rc::new(updated_card))
        }
        Duration::UntilClosed => Some(card.clone()),
        Duration::None => None,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::rc::Rc;

    use uuid::Uuid;

    use crate::cards::types::card_model::Card;
    use crate::world::board::Board;

    pub fn generate_board_with_open_card(card: Card) -> (Uuid, Board, Rc<Card>) {
        let card_rc = Rc::new(card.clone());
        let card_id = Uuid::new_v4();

        let open_cards = vec![(card_id.clone(), card_rc.clone())];

        let board = Board {
            open_cards: open_cards.into_iter().collect(),
            ..Board::empty()
        };
        (card_id, board, card_rc)
    }
}
