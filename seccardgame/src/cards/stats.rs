use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use log::warn;
use game_lib::cards::model::{Card, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};
use game_lib::file::cards::get_card_directory;
use game_lib::file::general::count_files_in_directory_with_filter;
use crate::cli::cli_result::CliResult;
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
    pub fn create(cfg: &Config) -> CliResult<CardStats> {
        Ok(CardStats {
            event_cards: Self::count_event_cards(cfg),
            oopsie_cards: Self::count_oopsie_cards(cfg),
            lucky_cards: Self::count_lucky_cards(cfg),
            incident_cards: Self::count_incident_cards(cfg),
            targets: Self::read_targets(cfg),
        })
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

    fn read_targets(cfg: &Config) -> HashMap<String, Target> {
        // only incident and oopsie cards have targets
        let oopsie_targets = Self::read_oopsie_targets(cfg);
        let incident_targets = Self::read_incident_targets(cfg);
        let mut result = HashMap::new();

        for ot in oopsie_targets {
            if result.contains_key(&ot) {
                let old: &Target = result.get(&ot).unwrap();
                result.insert(ot, Target {
                    target: old.target.clone(),
                    oopsie: old.oopsie + 1,
                    incident: old.incident
                });
            } else {
                result.insert(ot.clone(), Target {
                    target: ot.clone(),
                    oopsie: 1,
                    incident: 0
                });
            }
        };
        for it in incident_targets {
            if result.contains_key(&it) {
                let old = result.get(&it).unwrap();
                result.insert(it, Target {
                    target: old.target.clone(),
                    oopsie: old.oopsie,
                    incident: old.incident + 1
                });
            } else {
                result.insert(it.clone(), Target {
                    target: it.clone(),
                    oopsie: 0,
                    incident: 1
                });
            }
        }
        result
    }

    fn read_oopsie_targets(cfg: &Config) -> Vec<String> {
        let oopsie_card = Card::Oopsie(OopsieCard {
            title: "".to_string(),
            description: "".to_string(),
            targets: vec![],
            action: "".to_string(),
            fix_cost: FixCost { min: 0, max: 0 },
        });
        let mut path = PathBuf::from(&cfg.game_path.as_str());
        path.push(get_card_directory(&oopsie_card));
        let mut oopsie_targets = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let file = entry.unwrap();
            if file.metadata().unwrap().is_file() && file.file_name().to_str().unwrap().contains(".json") {
                let content = fs::read_to_string(file.path().to_str().unwrap()).unwrap();
                let card: OopsieCard = serde_json::from_str(content.as_str()).unwrap();
                oopsie_targets.extend(card.targets);
            }
        };
        oopsie_targets
    }

    fn read_incident_targets(cfg: &Config) -> Vec<String> {
        let incident_card = Card::Incident(IncidentCard {
            title: "".to_string(),
            description: "".to_string(),
            targets: vec![],
            action: "".to_string(),
        });
        let mut path = PathBuf::from(&cfg.game_path.as_str());
        path.push(get_card_directory(&incident_card));
        let mut incident_targets = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let file = entry.unwrap();
            if file.metadata().unwrap().is_file() && file.file_name().to_str().unwrap().contains(".json") {
                let content = fs::read_to_string(file.path().to_str().unwrap()).unwrap();
                let card: IncidentCard = serde_json::from_str(content.as_str()).unwrap();
                incident_targets.extend(card.targets);
            }
        };
        incident_targets
    }

}

pub(crate) fn print_stats(cfg: &Config) -> CliResult<()> {
    let stats = CardStats::create(cfg)?;
    println!("======Card Stats=====");
    println!("Events:\t\t{}",stats.event_cards);
    println!("Lucky:\t\t{}",stats.lucky_cards);
    println!("Oopsie:\t\t{}",stats.oopsie_cards);
    println!("Incident:\t{}",stats.incident_cards);
    println!("=====Targets=====");
    if stats.targets.len() > 0 {
        println!("{:<20}\t\tOopsie\tIncident", "Name");
        for target in stats.targets {
            let tgt = target.1;
            let target_name = truncate_string(tgt.target, 20);

            println!("{:<20}\t\t{}\t{}", target_name, tgt.oopsie, tgt.incident)
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

