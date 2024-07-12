use std::rc::Rc;

use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;

/// `Deck` is a struct that represents a deck of cards.
///
/// `board` is a vector of `Card` objects representing the cards in the deck. It is public to allow direct access to the cards.
///
/// `played_cards` is an integer representing the number of cards that have been played from the deck.
///
/// `total` is an integer representing the total number of cards in the deck.
///
/// # Examples
///
/// ```rust
/// use my_library::Deck;
///
/// let mut deck = Deck {
///     board: vec![],
///     played_cards: 0,
///     total: 52,
/// };
/// println!("{:?}", deck);
/// ```
/// This will create a new deck with an empty board, no played cards, and a total of 52 cards. It will then print the deck using debug formatting.
#[derive(Debug, Clone)]
pub struct Deck {
    pub remaining_cards: Vec<Card>,
    pub played_cards: usize,
    pub total: usize,
}

impl Deck {
    /// Creates a new `Deck` with the given `cards`.
    ///
    /// The `Deck` struct represents a playing deck, with a collection of `Card` objects.
    /// The `total` field represents the total number of cards in the deck, and the `played_cards` field
    /// represents the number of cards that have been played from the deck.
    ///
    /// # Arguments
    ///
    /// * `cards` - A vector containing the initial set of cards for the deck.
    ///
    /// # Returns
    ///
    /// A new `Deck` object initialized with the provided `cards`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Deck;
    /// use crate::Card;
    ///
    /// let cards = vec![
    ///     Card { value: 2, suit: "hearts".to_string() },
    ///     Card { value: 5, suit: "clubs".to_string() },
    ///     Card { value: 10, suit: "spades".to_string() },
    /// ];
    ///
    /// let deck = Deck::new(cards);
    /// ```
    fn new(cards: Vec<Card>) -> Deck {
        let total = cards.len();
        Deck {
            remaining_cards: cards,
            played_cards: 0,
            total,
        }
    }

    /// Returns a new instance of `Deck` with the given `cards` replacing the `board` field.
    ///
    /// # Arguments
    ///
    /// * `cards` - A slice of `Card` objects representing the new board configuration.
    ///
    /// # Returns
    ///
    /// A new `Deck` instance with the updated `board` field and incremented `played_cards` field.
    pub(crate) fn with_remaining_cards(&self, cards: &[Card]) -> Self {
        Deck {
            remaining_cards: cards.to_vec(),
            played_cards: self.played_cards + 1,
            total: self.total,
        }
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
}

pub struct DeckComposition {
    pub events: usize,
    pub attacks: usize,
    pub oopsies: usize,
    pub lucky: usize,
}

impl Into<Card> for EventCards {
    fn into(self) -> Card {
        match self {
            EventCards::Event(c) => Card::Event(c.clone()),
            EventCards::Oopsie(c) => Card::Oopsie(c.clone()),
            EventCards::Lucky(c) => Card::Lucky(c.clone()),
        }
    }
}

impl Into<Card> for AttackCards {
    fn into(self) -> Card {
        match self {
            AttackCards::Attack(c) => Card::Attack(c.clone()),
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

pub trait DeckPreparation {
    fn prepare<T: DeckRepository>(composition: DeckComposition, access: T) -> PreparedDeck;
    fn shuffle(&self) -> Deck;
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
        PreparedDeck {
            cards,
            attacks: draw_attack_cards_for_deck(composition.attacks, total_attack_cards),
        }
    }

    fn shuffle(&self) -> Deck {
        let mut rng = thread_rng();
        let total = &self.cards.len() + &self.attacks.len();
        let event_cards = &self.cards;
        let attack_cards = &self.attacks;
        let mut cards: Vec<Card> = event_cards.iter().map(|c| c.clone().into()).collect();

        cards.shuffle(&mut rng);

        let (no_attack_cards, to_have_attack_cards) = cards.split_at(total / 4);

        let mut part_with_attacks: Vec<Card> = to_have_attack_cards.to_vec();
        part_with_attacks.extend(
            attack_cards
                .iter()
                .map(|c| c.clone().into())
                .collect::<Vec<_>>(),
        );
        part_with_attacks.shuffle(&mut rng);

        cards = no_attack_cards.to_vec().clone();
        cards.extend(part_with_attacks);
        Deck::new(cards)
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
        match *cards_available[card_to_include].as_ref() {
            Card::Attack(ref c) => cards.push(AttackCards::Attack(c.clone())),
            _ => {}
        }
        x += 1;
    }
    cards
}
