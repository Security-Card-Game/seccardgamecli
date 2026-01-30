use game_setup::config::config::Config;
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};

pub fn open_ui(config: &Config) -> CliResult<()> {

    game_ui::start::run(None, config.clone()).map_err(|e| CliError {
        kind: ErrorKind::GUI,
        message: "Could not open GUI".to_string(),
        original_message: Some(e.to_string()),
    })
}
