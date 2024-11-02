use dialoguer::{Confirm, Editor, Select};
use log::error;

use game_lib::cards::properties::description::Description;
use game_lib::cards::properties::duration::Duration;
use game_lib::cards::properties::effect::Effect;
use game_lib::cards::properties::effect_description::EffectDescription;
use game_lib::cards::properties::fix_cost::FixCost;
use game_lib::cards::properties::fix_modifier::FixModifier;
use game_lib::cards::properties::target::Target;
use game_lib::cards::properties::title::Title;
use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::Card;
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;
use game_lib::world::resources::Resources;

use crate::cards::stats::print_stats;
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::cli_result::ErrorKind::FileSystemError;
use crate::cli::config::Config;
use crate::cli::prompts::prompt;

fn write_card_to_file(card: &Card, cfg: &Config) -> CliResult<()> {
    let mut card_to_save: Card = card.clone();
    if Confirm::new()
        .with_prompt("Do you want to edit this card?")
        .interact()
        .unwrap()
    {
        card_to_save =
            match Editor::new().edit(serde_json::to_string_pretty(card).unwrap().as_str()) {
                Ok(content) => match content {
                    Some(edited) => deserialize_editor_content(edited, &card).unwrap_or_else(|e| {
                        error!("Error deserializing into card: {}", e);
                        card_to_save
                    }),
                    None => card_to_save,
                },
                Err(e) => {
                    return Err(CliError::new(
                        ErrorKind::CardError,
                        "Could not edit card",
                        Some(e.to_string()),
                    ))
                }
            };
        println!(
            "Card to save is\n{}",
            serde_json::to_string_pretty(&card_to_save).unwrap()
        );
    }
    if Confirm::new()
        .with_prompt("Do you confirm these details?")
        .interact()
        .unwrap()
    {
        game_lib::file::cards::write_card_to_file(&card_to_save, Some(cfg.game_path.as_str()))
            .map_err(|e| {
                CliError::new(
                    FileSystemError,
                    "Could not write to file!",
                    Some(e.to_string()),
                )
            })?;
    } else {
        println!("Cancelled!");
    }
    Ok(())
}

fn deserialize_editor_content(content: String, original_card: &Card) -> serde_json::Result<Card> {
    match original_card {
        Card::Event(_) => serde_json::from_str(content.as_str()).map(|c| Card::Event(c)),
        Card::Attack(_) => serde_json::from_str(content.as_str()).map(|c| Card::Attack(c)),
        Card::Oopsie(_) => serde_json::from_str(content.as_str()).map(|c| Card::Oopsie(c)),
        Card::Lucky(_) => serde_json::from_str(content.as_str()).map(|c| Card::Lucky(c)),
        Card::Evaluation(_) => panic!("Cannot deserialize evaluation card"),
    }
}

pub fn create(cfg: &Config) -> CliResult<()> {
    print_stats(cfg)?;
    println!();
    let card_type_index = Select::new()
        .with_prompt("Select a card type to create")
        .items(&Card::CARD_TYPES)
        .default(0)
        .interact()
        .unwrap();

    let card = match Card::CARD_TYPES[card_type_index] {
        Card::EVENT_CARD => create_event_card(),
        Card::ATTACK_CARD => create_attack_card(),
        Card::LUCKY_CARD => create_lucky_card(),
        Card::OOPSIE_CARD => create_oopsie_card(),
        _ => {
            return Err(CliError::new(
                ErrorKind::CardError,
                "Unknown card type",
                None,
            ))
        }
    };

    write_card_to_file(&card, cfg)
}

fn create_event_card() -> Card {
    println!("Create a new Event Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);

    let effect = ask_for_modifying_effect();

    let card = EventCard {
        title: Title::from(title),
        description: Description::from(description),
        effect,
    };

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Event(card)
}

fn ask_for_modifying_effect() -> Effect {
    let effect_desc: String = prompt("Card Effect", None);
    let description = EffectDescription::new(&effect_desc);

    if Confirm::new()
        .with_prompt("Does this card alter fix costs?")
        .interact()
        .unwrap()
    {
        let time_point = ["next fix", "when used"];
        let selection = Select::new()
            .with_prompt("When does it alter it?")
            .items(&time_point)
            .default(0)
            .interact()
            .unwrap();

        let amount: isize = prompt("How much does it affect it?", None);
        let fix_modifier = if amount >= 0 {
            FixModifier::Increase(Resources::new(amount.abs() as usize))
        } else {
            FixModifier::Decrease(Resources::new(amount.abs() as usize))
        };

        match selection {
            0 => Effect::OnNextFix(description, fix_modifier.clone()),
            1 => Effect::OnUsingForFix(description, fix_modifier.clone()),
            _ => Effect::Other(description),
        }
    } else {
        Effect::Other(description)
    }
}

fn create_attack_card() -> Card {
    println!("Create a new Attack Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let effect: String = prompt("Card Effect", None);
    let duration: usize = prompt("Duration (rounds)", None);

    let targets = ask_for_targets();

    let card = AttackCard::new(
        Title::from(title),
        Description::from(description),
        targets.iter().map(|t| Target::from(t.clone())).collect(),
        EffectDescription::from(effect),
        Duration::Rounds(duration),
    );

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Attack(card)
}

fn ask_for_targets() -> Vec<String> {
    println!("Add targets of this card, enter a blank target when finished");
    let mut targets: Vec<String> = Vec::new();
    let available_targets = [
        "backend",
        "backup",
        "database",
        "frontend",
        "hardware",
        "infrastructure",
        "mobile",
        "network",
        "social",
        "supply chain",
        "Finished",
    ];
    loop {
        let selection = Select::new()
            .with_prompt("Add target (use Finished to end)")
            .items(&available_targets)
            .default(0)
            .interact()
            .unwrap();
        let target = available_targets.get(selection).unwrap().to_string();
        if target == "Finished" {
            break;
        }
        targets.push(target);
    }
    targets
}

fn create_lucky_card() -> Card {
    println!("Create a new Lucky Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let effect = ask_for_modifying_effect();

    let card = LuckyCard {
        title: Title::from(title),
        description: Description::from(description),
        effect,
    };

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Lucky(card)
}

fn create_oopsie_card() -> Card {
    println!("Create a new Oopsie Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let effect: String = prompt("Card Effect", None);
    let targets = ask_for_targets();
    let mut min_cost: usize;
    let mut max_cost: usize;
    loop {
        min_cost = prompt("Minimal fixing costs", None);
        max_cost = prompt("Maximal fixing costs", None);
        if min_cost <= max_cost {
            break;
        }
        println!("Max cost must be greater or equal to min cost.")
    }
    let card = OopsieCard::new(
        Title::from(title),
        Description::from(description),
        targets.iter().map(|t| Target::from(t.clone())).collect(),
        EffectDescription::from(effect),
        FixCost {
            min: Resources::new(min_cost),
            max: Resources::new(max_cost),
        },
    );

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Oopsie(card)
}
