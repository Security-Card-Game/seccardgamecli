use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;
use crate::game::create::create_deck;

pub fn play_deck(config: &Config) -> CliResult<()> {
    let deck = create_deck(config);

    game_ui::start::run(deck).map_err(|e| CliError {
        kind: ErrorKind::GUI,
        message: "Could not open GUI".to_string(),
        original_message: Some(e.to_string()),
    })
}
