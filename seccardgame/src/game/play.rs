
use game_lib::file::general::get_files_in_directory_with_filter;

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};

pub fn play_deck(deck_path: String) -> CliResult<()> {
    get_files_in_directory_with_filter(&deck_path, ".json").map_err(|e| CliError {
        kind: ErrorKind::FileSystemError,
        message: "Could not read deck".to_string(),
        original_message: Some(e.to_string()),
    })?;

    game_ui::start::run(deck_path).expect("Could not load GUI");
    Ok(())
}
