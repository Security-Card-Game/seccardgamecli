use game_lib::file::repository::DeckLoader;
use game_lib::world::deck::{Deck, DeckComposition, DeckPreparation, PreparedDeck};

use crate::config::config::Config;


pub fn create_deck(deck_composition: &DeckComposition, grace_period: u8, config: &Config) -> Deck {

    let prepared_deck = PreparedDeck::prepare(
        deck_composition,
        DeckLoader::create(config.game_path.as_str()),
    );

    prepared_deck.shuffle(grace_period as usize)
}