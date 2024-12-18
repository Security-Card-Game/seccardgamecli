use std::rc::Rc;
use log::warn;
use rand::{Rng, thread_rng};
use rand::prelude::{SliceRandom, ThreadRng};

use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;
use crate::cards::types::evaluation::EvaluationCard;


/// This represents the current deck of cards. It also keeps count of the already played cards and the
/// remaining cards. This file also contains all the methods needed to create a new Deck. E.g shuffling the cards
/// and a helper struct `PreparedDeck`.
///
/// The Deck is not mutated in place. When a card is drawn a new Deck object is created.

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    pub remaining_cards: Vec<Card>,
    pub played_cards: usize,
    pub total: usize,
}

impl Deck {
    pub(crate) fn new(cards: Vec<Card>) -> Deck {
        let total = cards.len();
        Deck {
            remaining_cards: cards,
            played_cards: 0,
            total,
        }
    }

    pub(crate) fn get_remaining_card_count(&self) -> usize {
        self.remaining_cards.len()
    }
}

#[derive(Clone)]
pub enum EventCards {
    Event(EventCard),
    Oopsie(OopsieCard),
    Lucky(LuckyCard),
}

#[derive(Clone)]
pub enum AttackCards {
    Attack(AttackCard),
}

pub struct PreparedDeck {
    cards: Vec<EventCards>,
    attacks: Vec<AttackCards>,
    evaluation: Vec<EvaluationCard>,
}

pub struct DeckComposition {
    pub events: usize,
    pub attacks: usize,
    pub oopsies: usize,
    pub lucky: usize,
    pub evaluation: usize
}

impl From<EventCards> for Card {
    fn from(value: EventCards) -> Self {
        match value {
            EventCards::Event(ec) => Card::Event(ec),
            EventCards::Oopsie(oc) => Card::Oopsie(oc),
            EventCards::Lucky(lc) => Card::Lucky(lc),
        }
    }
}

impl From<AttackCards> for Card {
    fn from(value: AttackCards) -> Self {
        match value {
            AttackCards::Attack(ac) => Card::Attack(ac),
        }
    }
}

pub type CardRc = Rc<Card>;

pub trait DeckRepository {
    fn get_event_cards(&self) -> Vec<CardRc>;
    fn get_lucky_cards(&self) -> Vec<CardRc>;
    fn get_oopsie_cards(&self) -> Vec<CardRc>;
    fn get_attack_cards(&self) -> Vec<CardRc>;
}

/// Defines a trait for deck preparation.
pub trait DeckPreparation {
    /// Assembles a `PreparedDeck` by pulling the specified number of cards from a `DeckRepository`
    /// based on the given `DeckComposition`.
    fn prepare<T: DeckRepository>(composition: DeckComposition, access: T) -> PreparedDeck;

    /// Shuffles the deck and inserts attack cards only after a grace period of cards/turns
    fn shuffle(&self, grace_period: usize) -> Deck;
}

impl DeckPreparation for PreparedDeck {
    fn prepare<T: DeckRepository>(composition: DeckComposition, access: T) -> PreparedDeck {
        dbg!("Creating a new deck");
        let mut cards: Vec<EventCards> = vec![];

        let total_event_cards = access.get_event_cards();
        let event_cards = draw_event_cards_for_deck(composition.events, total_event_cards);
        cards.append(&mut event_cards.clone());

        let total_oopsie_cards = access.get_oopsie_cards().to_vec();
        let oopsie_cards = draw_event_cards_for_deck(composition.oopsies, total_oopsie_cards);
        cards.append(&mut oopsie_cards.clone());

        let total_lucky_cards = access.get_lucky_cards().to_vec();
        let lucky_cards = draw_event_cards_for_deck(composition.lucky, total_lucky_cards);
        cards.append(&mut lucky_cards.clone());

        let total_attack_cards = access.get_attack_cards().to_vec();

        let evaluation_cards = (0..composition.evaluation).map(|_| EvaluationCard::default()).collect();

        PreparedDeck {
            cards,
            attacks: draw_attack_cards_for_deck(composition.attacks, total_attack_cards),
            evaluation: evaluation_cards,
        }
    }

    fn shuffle(&self, grace_period: usize) -> Deck {
        let mut rng = thread_rng();
        let total = self.cards.len() + self.attacks.len();

        let attack_graces = if grace_period >= total {
            let fallback = total / 4;
            warn!("Grace period must be < cards count. Defaulting to {}.", fallback);
            fallback
        } else {
            grace_period
        };

        let event_cards = &self.cards;
        let attack_cards = &self.attacks;
        let mut cards: Vec<Card> = event_cards.iter().map(|c| c.clone().into()).collect();
        cards.shuffle(&mut rng);

        let cards = Self::add_attack_cards(&mut rng, attack_graces, attack_cards, cards);
        let cards = Self::add_evaluation_cards(&mut rng, &self.evaluation, cards);
        
        Deck::new(cards)
    }
}

impl PreparedDeck {
    fn add_attack_cards(mut rng: &mut ThreadRng, attack_graces: usize, attack_cards: &Vec<AttackCards>, cards: Vec<Card>) -> Vec<Card> {
        let (no_attack_cards, to_have_attack_cards) = cards.split_at(attack_graces);

        let mut part_with_attacks: Vec<Card> = to_have_attack_cards.to_vec();
        part_with_attacks.extend(
            attack_cards
                .iter()
                .map(|c| c.clone().into())
                .collect::<Vec<_>>(),
        );
        part_with_attacks.shuffle(&mut rng);

        let mut cards_without_attacks = no_attack_cards.to_vec().clone();
        cards_without_attacks.extend(part_with_attacks);
        cards_without_attacks
    }

    /// This function inserts evaluation cards at regular intervals into a given vector of cards.
    /// It divides the cards into chunks, adds an evaluation card to all chunks except the first,
    /// shuffles the chunks with the new cards, and then consolidates them back into a single vector.
    // TODO: Add a test for this method
    fn add_evaluation_cards(mut rng: &mut ThreadRng, evaluation_cards: &Vec<EvaluationCard>, cards: Vec<Card>) -> Vec<Card> {
        let eval_count = evaluation_cards.len();
        let chunk_size = cards.len() / (eval_count + 1);
        let chunks = cards.chunks(chunk_size);
        
        let mut cards = Vec::new();
        for (i, chunk) in chunks.enumerate() {
            if i == 0 {
                cards.extend(chunk.to_vec());
                continue
            }
            let mut chunk_with_eval = chunk.to_vec();
            if i - 1 < eval_count {
                chunk_with_eval.push(evaluation_cards[i - 1].clone().into());
            }
            chunk_with_eval.shuffle(&mut rng);
            cards.extend(chunk_with_eval)
        }
        
        cards
    }

}

fn draw_event_cards_for_deck(count: usize, cards_available: Vec<CardRc>) -> Vec<EventCards> {
    let mut x = 0;
    let mut cards = vec![];

    while x < count {
        let card_to_include = thread_rng().gen_range(0..cards_available.len());
        match *cards_available[card_to_include].as_ref() {
            Card::Event(ref c) => cards.push(EventCards::Event(c.clone())),
            Card::Attack(_) => {}
            Card::Oopsie(ref c) => cards.push(EventCards::Oopsie(c.clone())),
            Card::Lucky(ref c) => cards.push(EventCards::Lucky(c.clone())),
            Card::Evaluation(_) => {},
        }
        x += 1;
    }
    cards
}

fn draw_attack_cards_for_deck(count: usize, cards_available: Vec<CardRc>) -> Vec<AttackCards> {
    let mut x = 0;
    let mut cards = vec![];

    while x < count {
        let card_to_include = thread_rng().gen_range(0..cards_available.len());
        if let Card::Attack(ref c) = *cards_available[card_to_include].as_ref() {
            cards.push(AttackCards::Attack(c.clone()))
        }
        x += 1;
    }
    cards
}
