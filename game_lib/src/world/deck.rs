use crate::cards::types::card_model::{Card, CardCategory, CardTrait};
use crate::cards::types::evaluation::EvaluationCard;
use log::warn;
use rand::prelude::{SliceRandom, ThreadRng};
use std::rc::Rc;
use rand::thread_rng;
use crate::cards::game_variants::scenario::Scenario;

/// This represents the current deck of cards. It also keeps count of the already played cards and the
/// remaining cards. This file also contains all the methods needed to create a new Deck. E.g shuffling the cards
/// and a helper struct `PreparedDeck`.
///
/// The Deck is not mutated in place. When a card is drawn a new Deck object is created.

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    pub remaining_cards: Vec<CardRc>,
    pub played_cards: usize,
    pub total: usize,
}


impl Deck {
    pub(crate) fn new(cards: Vec<CardRc>) -> Deck {
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

pub struct PreparedDeck {
    cards: Vec<CardRc>,
    attacks: Vec<CardRc>,
    evaluation: Vec<EvaluationCard>,
}

#[derive(Debug, Clone, Copy)]
pub struct DeckComposition {
    pub events: usize,
    pub attacks: usize,
    pub oopsies: usize,
    pub lucky: usize,
    pub evaluation: usize,
}
pub type CardRc = Rc<Card>;

pub trait DeckRepository {
    fn get_event_cards(&self) -> Vec<CardRc>;
    fn get_lucky_cards(&self) -> Vec<CardRc>;
    fn get_oopsie_cards(&self) -> Vec<CardRc>;
    fn get_attack_cards(&self) -> Vec<CardRc>;
}

pub trait GameVariantsRepository {
    fn get_scenarios(&self) -> Vec<Rc<Scenario>>;
}

/// Defines a trait for deck preparation.
pub trait DeckPreparation {
    /// Assembles a `PreparedDeck` by pulling the specified number of cards from a `DeckRepository`
    /// based on the given `DeckComposition`.
    fn prepare<T: DeckRepository>(composition: &DeckComposition, access: T) -> PreparedDeck;

    /// Shuffles the deck and inserts attack cards only after a grace period of cards/turns
    fn shuffle(&self, grace_period: usize) -> Deck;
}

impl DeckPreparation for PreparedDeck {
    fn prepare<T: DeckRepository>(composition: &DeckComposition, access: T) -> PreparedDeck {
        dbg!("Creating a new deck");
        let mut cards: Vec<CardRc> = vec![];

        let total_event_cards = access.get_event_cards();

        let event_cards = draw(composition.events, total_event_cards, &Card::EVENT_CARD)
        .map_err(|e| format!("Event Cards: {}", e).to_string())
        .unwrap();
        cards.append(&mut event_cards.clone());

        let total_oopsie_cards = access.get_oopsie_cards().to_vec();
        let oopsie_cards = draw(composition.oopsies, total_oopsie_cards, &Card::OOPSIE_CARD)
        .map_err(|e| format!("Oopsie Cards: {}", e).to_string())
        .unwrap();
        cards.append(&mut oopsie_cards.clone());

        let total_lucky_cards = access.get_lucky_cards().to_vec();
        let lucky_cards = draw(composition.lucky, total_lucky_cards, &Card::LUCKY_CARD)
        .map_err(|e| format!("Lucky Cards: {}", e).to_string())
        .unwrap();
        cards.append(&mut lucky_cards.clone());

        let total_attack_cards = access.get_attack_cards().to_vec();
        let attack_cards = draw(composition.attacks, total_attack_cards, &Card::ATTACK_CARD)
        .map_err(|e| format!("Attack Cards: {}", e).to_string())
        .unwrap();

        let evaluation_cards = (0..composition.evaluation)
            .map(|_| EvaluationCard::default())
            .collect();

        PreparedDeck {
            cards,
            attacks: attack_cards,
            evaluation: evaluation_cards,
        }
    }

    fn shuffle(&self, grace_period: usize) -> Deck {
        let mut rng = thread_rng();
        let total = self.cards.len() + self.attacks.len();

        let attack_graces = if grace_period >= total {
            let fallback = total / 4;
            warn!(
                "Grace period must be < cards count. Defaulting to {}.",
                fallback
            );
            fallback
        } else {
            grace_period
        };

        let normal_cards = &self.cards.clone();
        let attack_cards = &self.attacks;
        let mut cards: Vec<CardRc> = normal_cards.iter().map(|c| c.clone()).collect();
        cards.shuffle(&mut rng);

        let cards = Self::add_attack_cards(&mut rng, attack_graces, attack_cards, cards);
        let cards = Self::add_evaluation_cards(&mut rng, &self.evaluation, cards);

        Deck::new(cards)
    }
}

impl PreparedDeck {
    fn add_attack_cards(
        mut rng: &mut ThreadRng,
        attack_graces: usize,
        attack_cards: &Vec<CardRc>,
        cards: Vec<CardRc>,
    ) -> Vec<CardRc> {
        let (no_attack_cards, to_have_attack_cards) = cards.split_at(attack_graces);

        let mut part_with_attacks: Vec<CardRc> = to_have_attack_cards.to_vec();
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
    fn add_evaluation_cards(
        mut rng: &mut ThreadRng,
        evaluation_cards: &Vec<EvaluationCard>,
        cards: Vec<CardRc>,
    ) -> Vec<CardRc> {
        let eval_count = evaluation_cards.len();
        let chunk_size = cards.len() / (eval_count + 1);
        let chunks = cards.chunks(chunk_size);

        let mut cards = Vec::new();
        for (i, chunk) in chunks.enumerate() {
            if i == 0 {
                cards.extend(chunk.to_vec());
                continue;
            }
            let mut chunk_with_eval = chunk.to_vec();
            if i - 1 < eval_count {
                chunk_with_eval.push(
                    Rc::new(Card::from(evaluation_cards[i - 1].clone()))
                );
            }
            chunk_with_eval.shuffle(&mut rng);
            cards.extend(chunk_with_eval)
        }

        cards
    }
}

fn draw(count: usize, cards: Vec<CardRc>, category: &CardCategory) -> Result<Vec<CardRc>, String> {
    fn extract(card: &CardRc, cat: &CardCategory) ->  Option<CardRc>
    {
        if card.category() == cat {
            Some(card.clone())
        } else {
            None
        }
    }


    let cards_to_use = cards
        .iter()
        .map(|c| extract(c, category) )
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    if cards_to_use.is_empty() {
        return Err("No cards to draw from".to_string());
    }

    if cards_to_use.len() < count {
        let card_type = cards_to_use[0].category();
        warn!(
            "Not enough {} cards to draw {} cards from. Will duplicate!",
            card_type, count
        );
    }

    let mut cards_to_draw_from: Vec<CardRc> = vec![];
    while cards_to_draw_from.len() < count {
        let mut shuffled = cards_to_use.clone();
        shuffled.shuffle(&mut thread_rng());
        cards_to_draw_from.extend(shuffled);
    }

    let ret_val = cards_to_draw_from.iter().take(count).cloned().collect();
    Ok(ret_val)
}

#[cfg(test)]
mod tests {
    use std::collections;
    use fake::Fake;
    use collections::HashSet;
    use crate::cards::types::event::EventCard;
    use crate::cards::types::event::tests::FakeEventCard;
    use crate::cards::types::lucky::tests::FakeLuckyCard;
    use super::*;

    fn extract_cards_from_result(res: &Vec<CardRc>) -> Vec<&EventCard> {
        res.iter().map(|c| match &**c {
            Card::Event(ec) => ec,
            _ => panic!("Only event card expected in result. Got: {:?}", c),
        })
            .collect()
    }


    #[test]
    fn draw_from_no_valid_cards_should_result_in_error() {
        let count: usize = 3;
        let cards: Vec<CardRc> = vec![
            Rc::new(Card::Lucky(FakeLuckyCard.fake())),
            Rc::new(Card::Lucky(FakeLuckyCard.fake())),
            Rc::new(Card::Lucky(FakeLuckyCard.fake()))
        ];

        assert!(draw(count, cards, &Card::EVENT_CARD).is_err());
    }

    #[test]
    fn draw_from_not_enough_cards_should_result_in_duplicates() {
        let count: usize = 3;

        let cards: Vec<CardRc> = vec![
            Rc::new(Card::Event(FakeEventCard.fake()))
        ];

        let res = draw(count, cards, &Card::EVENT_CARD)
            .unwrap();

        let drawn_cards = extract_cards_from_result(&res);

        assert_eq!(drawn_cards.len(), count);
        // assert if only on card title is present
        let set: HashSet<&str> = drawn_cards.iter().map(|c| c.title.value()).collect();
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn draw_from_enough_cards_should_result_in_unique_cards() {
        let count: usize = 3;
        let cards: Vec<CardRc> = vec![
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake()))
        ];

        let res = draw(count, cards, &Card::EVENT_CARD)
            .unwrap();

        let drawn_cards = extract_cards_from_result(&res);

        assert_eq!(drawn_cards.len(), count);
        // assert no duplicate title is present
        let set: HashSet<&str> = drawn_cards.iter().map(|c| c.title.value()).collect();
        assert_eq!(set.len(), 3);
    }

    #[test]
    /*
    Assumption: Two successive draws should result in different cards.
    */
    fn draw_should_be_shuffled() {
        let count: usize = 3;
        let cards: Vec<CardRc> = vec![
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
            Rc::new(Card::Event(FakeEventCard.fake())),
        ];

        let draw_1 = draw(count, cards.clone(), &Card::EVENT_CARD).unwrap();
        let draw_2 = draw(count, cards, &Card::EVENT_CARD).unwrap();

        let drawn_cards_1 = extract_cards_from_result(&draw_1);
        let drawn_cards_2 = extract_cards_from_result(&draw_2);

        let titles_1: Vec<&str> = drawn_cards_1.iter().map(|c| c.title.value()).collect();
        let titles_2: Vec<&str> = drawn_cards_2.iter().map(|c| c.title.value()).collect();

        assert_ne!(titles_1, titles_2);
    }


}
