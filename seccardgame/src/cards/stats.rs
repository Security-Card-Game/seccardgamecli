use std::collections::HashMap;
use std::path::PathBuf;
use log::warn;
use game_lib::cards::model::{Card, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};
use game_lib::file::cards::get_card_directory;
use game_lib::file::general::count_files_in_directory_with_filter;
use crate::cli::config::Config;

pub struct CardStats {
    pub event_cards: u32,
    pub incident_cards: u32,
    pub oopsie_cards: u32,
    pub lucky_cards: u32,
    pub targets: HashMap<String, Target>,
}

pub struct Target {
    pub target: String,
    pub incident: u32,
    pub oopsie: u32,
}

impl CardStats {
    pub fn create(cfg: &Config) -> Self {
        CardStats {
            event_cards: Self::count_event_cards(cfg),
            oopsie_cards: Self::count_oopsie_cards(cfg),
            lucky_cards: Self::count_lucky_cards(cfg),
            incident_cards: Self::count_incident_cards(cfg),
            targets: HashMap::new(),
        }
    }

    fn count_event_cards(cfg: &Config) -> u32 {
        let card = Card::Event(EventCard {
            title: "".to_string(),
            description: "".to_string(),
            action: "".to_string(),
        });
        Self::count_files(&cfg, &card)
    }

    fn count_oopsie_cards(cfg: &Config) -> u32 {
        let card = Card::Oopsie(OopsieCard {
            title: "".to_string(),
            description: "".to_string(),
            targets: vec![],
            action: "".to_string(),
            fix_cost: FixCost { min: 0, max: 0 },
        });
        Self::count_files(&cfg, &card)
    }

    fn count_incident_cards(cfg: &Config) -> u32 {
        let card = Card::Incident(IncidentCard {
            title: "".to_string(),
            description: "".to_string(),
            targets: vec![],
            action: "".to_string(),
        });
        Self::count_files(&cfg, &card)
    }

    fn count_lucky_cards(cfg: &Config) -> u32 {
        let card = Card::Lucky(LuckyCard {
            title: "".to_string(),
            description: "".to_string(),
            action: "".to_string(),
        });
        Self::count_files(&cfg, &card)
    }

    fn count_files(cfg: &Config, event_card: &Card) -> u32 {
        let filter = ".json";
        let mut base_path = PathBuf::from(&cfg.game_path);
        let card_dir = get_card_directory(&event_card);
        base_path.push(card_dir);
        let path = base_path.to_str().unwrap().trim();
        count_files_in_directory_with_filter(path, filter).unwrap_or_else(|e| {
            warn!("Error reading files for stats from {}: {}", path, e);
            0
        })
    }
}

pub(crate) fn print_stats(cfg: &Config) {
    let stats = CardStats::create(cfg);
    println!("======Card Stats=====");
    println!("Events:\t\t{}",stats.event_cards);
    println!("Lucky:\t\t{}",stats.lucky_cards);
    println!("Oopsie:\t\t{}",stats.oopsie_cards);
    println!("Incident:\t{}",stats.incident_cards);
}