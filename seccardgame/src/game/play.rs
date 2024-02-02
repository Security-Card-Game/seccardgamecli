use std::fs;
use std::io::stdout;

use crossterm::terminal::ClearType;
use crossterm::{terminal, ExecutableCommand};
use dialoguer::Confirm;

use game_lib::file::general::get_files_in_directory_with_filter;

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};

pub fn play_deck(deck_path: String) -> CliResult<()> {
    let cards = get_files_in_directory_with_filter(&deck_path, ".json").map_err(|e| CliError {
        kind: ErrorKind::FileSystemError,
        message: "Could not read deck".to_string(),
        original_message: Some(e.to_string()),
    })?;

    for (round, card) in cards.iter().enumerate() {
        clear_term()?;

        println!("Round {}", round);
        println!();

        let current_card = fs::read_to_string(card).map_err(|_| CliError {
            kind: ErrorKind::FileSystemError,
            message: format!("Could not read card {}", card.to_str().unwrap()).to_string(),
            original_message: None,
        })?;

        println!("{}", current_card);

        if !Confirm::new()
            .with_prompt("Continue?")
            .default(true)
            .interact()
            .unwrap()
        {
            break;
        }
    }
    Ok(())
}

fn clear_term() -> CliResult<()> {
    stdout()
        .execute(terminal::Clear(ClearType::All))
        .map_err(|e| CliError {
            kind: ErrorKind::UserInterfaceError,
            message: "Could clear terminal".to_string(),
            original_message: Some(e.to_string()),
        })?;

    Ok(())
}
