use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use log::warn;

use game_lib::cards::properties::effect::Effect;
use game_lib::cards::properties::target::Target;
use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::Card;
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;
use game_lib::file::cards::get_card_directory;
use game_lib::file::general::count_cards_in_directory;
use game_setup::config::config::Config;
use crate::cli::cli_result::CliResult;

pub struct CardStats {
    pub event_cards: u32,
    pub attack_cards: u32,
    pub oopsie_cards: u32,
    pub lucky_cards: u32,
    pub targets: HashMap<String, TargetCounter>,
}

pub struct TargetCounter {
    pub target: String,
    pub attack: u32,
    pub oopsie: u32,
}

impl CardStats {
    pub fn create(cfg: &Config) -> CliResult<CardStats> {
        Ok(CardStats {
            event_cards: Self::count_event_cards(cfg),
            oopsie_cards: Self::count_oopsie_cards(cfg),
            lucky_cards: Self::count_lucky_cards(cfg),
            attack_cards: Self::count_attack_cards(cfg),
            targets: Self::read_targets(cfg),
        })
    }

    fn count_event_cards(cfg: &Config) -> u32 {
        let card = EventCard::empty();
        Self::count_files(&cfg, &card)
    }

    fn count_oopsie_cards(cfg: &Config) -> u32 {
        let card = OopsieCard::empty();
        Self::count_files(&cfg, &card)
    }

    fn count_attack_cards(cfg: &Config) -> u32 {
        let card = AttackCard::empty();
        Self::count_files(&cfg, &card)
    }

    fn count_lucky_cards(cfg: &Config) -> u32 {
        let card = LuckyCard::empty();
        Self::count_files(&cfg, &card)
    }

    fn count_files(cfg: &Config, event_card: &Card) -> u32 {
        let mut base_path = PathBuf::from(&cfg.game_path);
        let card_dir = get_card_directory(&event_card);
        base_path.push(card_dir);
        let path = base_path.to_str().unwrap().trim();
        count_cards_in_directory(path).unwrap_or_else(|e| {
            warn!("Error reading files for stats from {}: {}", path, e);
            0
        })
    }

    fn read_targets(cfg: &Config) -> HashMap<String, TargetCounter> {
        // only attacks and oopsie types have targets
        let oopsie_targets = Self::read_oopsie_targets(cfg);
        let attack_targets = Self::read_attack_targets(cfg);
        let mut result = HashMap::new();

        for ot in oopsie_targets {
            let key = ot.value().to_string();
            if result.contains_key(&key) {
                let old: &TargetCounter = result.get(&key).unwrap();
                result.insert(
                    key.clone(),
                    TargetCounter {
                        target: old.target.clone(),
                        oopsie: old.oopsie + 1,
                        attack: old.attack,
                    },
                );
            } else {
                result.insert(
                    key.clone(),
                    TargetCounter {
                        target: key.clone(),
                        oopsie: 1,
                        attack: 0,
                    },
                );
            }
        }
        for it in attack_targets {
            let key = it.value().to_string();
            if result.contains_key(&key) {
                let old = result.get(&key).unwrap();
                result.insert(
                    key.clone(),
                    TargetCounter {
                        target: old.target.clone(),
                        oopsie: old.oopsie,
                        attack: old.attack + 1,
                    },
                );
            } else {
                result.insert(
                    key.clone(),
                    TargetCounter {
                        target: key.clone(),
                        oopsie: 0,
                        attack: 1,
                    },
                );
            }
        }
        result
    }

    fn read_oopsie_targets(cfg: &Config) -> Vec<Target> {
        let oopsie_card = OopsieCard::empty();
        let mut path = PathBuf::from(&cfg.game_path.as_str());
        path.push(get_card_directory(&oopsie_card));
        let mut oopsie_targets = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let file = entry.unwrap();
            if file.metadata().unwrap().is_file()
                && file.file_name().to_str().unwrap().contains(".json")
            {
                let content = fs::read_to_string(file.path().to_str().unwrap()).unwrap();
                let card: OopsieCard = serde_json::from_str(content.as_str()).unwrap();
                match card.effect {
                    Effect::AttackSurface(_, t) => oopsie_targets.extend(t),
                    _ => {}
                }
            }
        }
        oopsie_targets
    }

    fn read_attack_targets(cfg: &Config) -> Vec<Target> {
        let incident_card = AttackCard::empty();
        let mut path = PathBuf::from(&cfg.game_path.as_str());
        path.push(get_card_directory(&incident_card));
        let mut attack_targets = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let file = entry.unwrap();
            if file.metadata().unwrap().is_file()
                && file.file_name().to_str().unwrap().contains(".json")
            {
                let content = fs::read_to_string(file.path().to_str().unwrap()).unwrap();
                let card: AttackCard = serde_json::from_str(content.as_str()).unwrap();
                match card.effect {
                    Effect::Incident(_, t, _) => attack_targets.extend(t),
                    _ => {}
                }
            }
        }
        attack_targets
    }
}

pub(crate) fn print_stats(cfg: &Config) -> CliResult<()> {
    let stats = CardStats::create(cfg)?;
    println!("======Card Stats=====");
    println!("Events:\t\t{}", stats.event_cards);
    println!("Lucky:\t\t{}", stats.lucky_cards);
    println!("Oopsie:\t\t{}", stats.oopsie_cards);
    println!("Attacks:\t{}", stats.attack_cards);
    println!("=====Targets=====");
    if stats.targets.len() > 0 {
        println!("{:<20}\t\tOopsie\tIncident", "Name");
        for target in stats.targets {
            let tgt = target.1;
            let target_name = truncate_string(tgt.target, 20);

            println!("{:<20}\t\t{}\t{}", target_name, tgt.oopsie, tgt.attack)
        }
    } else {
        println!("No targets");
    }
    Ok(())
}
fn truncate_string(s: String, max_len: usize) -> String {
    if s.len() > max_len {
        let mut new_s = s;
        new_s.truncate(max_len - 3);
        new_s + "..."
    } else {
        s
    }
}
