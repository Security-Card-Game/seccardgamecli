use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::Card;
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

pub struct Deck {
    cards: Vec<Card>,
    total: usize,
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

pub trait DeckRepository {
    fn get_event_cards(&self) -> Vec<&Card>;
    fn get_lucky_cards(&self) -> Vec<&Card>;
    fn get_oopsie_cards(&self) -> Vec<&Card>;
    fn get_attack_cards(&self) -> Vec<&Card>;
}

pub trait DeckPreparation {
    fn prepare<T: DeckRepository>(composition: DeckComposition, access: T) -> PreparedDeck;
    fn shuffle(prepared_deck: PreparedDeck) -> Deck;
}

impl DeckPreparation for PreparedDeck {
    fn prepare<T: DeckRepository>(composition: DeckComposition, access: T) -> PreparedDeck {
        dbg!("Creating a new deck");
        let mut cards: Vec<EventCards> = vec![];

        let event_cards = draw_event_cards_for_deck(composition.events, access.get_event_cards());
        cards.append(&mut event_cards.clone());

        let oopsie_cards =
            draw_event_cards_for_deck(composition.oopsies, access.get_oopsie_cards());
        cards.append(&mut oopsie_cards.clone());

        let lucky_cards = draw_event_cards_for_deck(composition.lucky, access.get_lucky_cards());
        cards.append(&mut lucky_cards.clone());

        PreparedDeck {
            cards,
            attacks: draw_attack_cards_for_deck(composition.attacks, access.get_attack_cards()),
        }
    }

    fn shuffle(prepared_deck: PreparedDeck) -> Deck {
        let mut rng = thread_rng();
        let total = prepared_deck.cards.len() + prepared_deck.attacks.len();

        let mut cards: Vec<_> = prepared_deck
            .cards
            .into_iter()
            .map(|c| match c {
                EventCards::Event(c) => Card::Event(c),
                EventCards::Oopsie(c) => Card::Oopsie(c),
                EventCards::Lucky(c) => Card::Lucky(c),
            })
            .collect();
        cards.shuffle(&mut rng);

        let (safe_part, unsafe_part) = cards.split_at(total / 4);

        let mut part_with_attacks: Vec<Card> = unsafe_part.to_vec();
        part_with_attacks.extend(
            prepared_deck
                .attacks
                .into_iter()
                .map(|c| match c {
                    AttackCards::Attack(c) => Card::Attack(c),
                })
                .collect::<Vec<_>>(),
        );
        part_with_attacks.shuffle(&mut rng);

        cards = safe_part.to_vec().clone();
        cards.extend(part_with_attacks);
        Deck { cards, total }
    }
}

fn draw_event_cards_for_deck(count: usize, cards_available: Vec<&Card>) -> Vec<EventCards> {
    let mut x = 0;
    let mut cards = vec![];

    while x < count {
        let card_to_include = thread_rng().gen_range(0..cards_available.len());
        match cards_available[card_to_include] {
            Card::Event(c) => cards.push(EventCards::Event(c.clone())),
            Card::Attack(_) => {}
            Card::Oopsie(c) => cards.push(EventCards::Oopsie(c.clone())),
            Card::Lucky(c) => cards.push(EventCards::Lucky(c.clone())),
        }
    }
    cards
}

fn draw_attack_cards_for_deck(count: usize, cards_available: Vec<&Card>) -> Vec<AttackCards> {
    let mut x = 0;
    let mut cards = vec![];

    while x < count {
        let card_to_include = thread_rng().gen_range(0..cards_available.len());
        match cards_available[card_to_include] {
            Card::Attack(c) => cards.push(AttackCards::Attack(c.clone())),
            _ => {}
        }
    }
    cards
}
