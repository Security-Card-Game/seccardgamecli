use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use git2::Repository;
use log::info;
use serde::{Deserialize, Serialize};

use game_lib::file::general::ensure_directory_exists;

use crate::cli::cli_result::ErrorKind::{FileSystemError, GameCloneError};
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};

pub struct CfgInit {
    pub game_path: String,
    pub config_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub game_path: String,
}

const GAME_REPO: &str = "https://github.com/Security-Card-Game/securityDeckGame.git";
const DEFAULT_CFG: &str = "seccard_cfg.json";

pub fn init(init: CfgInit) -> CliResult<()> {
    init_impl(init, clone_game, create_config)
}

fn init_impl<F, G>(init: CfgInit, clone_game: F, create_config: G) -> CliResult<()>
where
    F: Fn(&str) -> Result<(), CliError>,
    G: Fn(Config, &str) -> Result<(), CliError>,
{
    clone_game(init.game_path.as_str())?;
    create_config(
        Config {
            game_path: init.game_path,
        },
        init.config_path.as_str(),
    )
}

impl Config {
    pub fn read_config(path_to_config: &str) -> Self {
        match fs::read_to_string(path_to_config) {
            Ok(cfg) => serde_json::from_str(cfg.as_str())
                .expect("Could not parse config file, did you run init?"),
            Err(e) => panic!(
                "Could not read config file {}, did you run init?",
                e.to_string()
            ),
        }
    }
}

fn create_config(cfg: Config, path_to_config: &str) -> CliResult<()> {
    let path = Path::new(path_to_config);
    let mut path_to_write = path.to_path_buf();
    if path.is_dir() {
        path_to_write.push(DEFAULT_CFG);
    } else {
        if let Some(dir) = path.parent() {
            match ensure_directory_exists(dir.to_str().unwrap().trim()) {
                Ok(_) => (),
                Err(e) => {
                    return Err(CliError {
                        kind: FileSystemError,
                        message: format!("Could not create directory {}", path.to_str().unwrap()),
                        original_message: Some(e.to_string()),
                    })
                }
            }
        }
    };
    match serde_json::to_string_pretty(&cfg) {
        Ok(json) => match write_config(json, path_to_write) {
            Ok(()) => {
                info!("Config file created");
                Ok(())
            }
            Err(e) => Err(e),
        },
        Err(_) => Err(CliError {
            kind: ErrorKind::ConfigError,
            message: "Could not serialize config!".to_string(),
            original_message: None,
        }),
    }
}

fn write_config(json: String, path: PathBuf) -> CliResult<()> {
    let mut file = File::create(path).map_err(|err| CliError {
        kind: FileSystemError,
        message: "Could not crate config file!".to_string(),
        original_message: Some(err.to_string()),
    })?;
    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(CliError {
            kind: FileSystemError,
            message: "Could not write config file!".to_string(),
            original_message: Some(e.to_string()),
        }),
    }
}

fn clone_game(path: &str) -> CliResult<()> {
    match ensure_directory_exists(path) {
        Ok(_) => (),
        Err(e) => {
            return Err(CliError {
                kind: FileSystemError,
                message: format!("Could not create directory {}", path),
                original_message: Some(e.to_string()),
            })
        }
    };
    info!("Cloning game repository...");
    match Repository::clone(GAME_REPO, path) {
        Ok(_) => (),
        Err(e) => {
            return Err(CliError {
                kind: GameCloneError,
                message: "Failed to clone game".to_string(),
                original_message: Some(e.to_string()),
            })
        }
    };
    info!("Downloaded game into {}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::cli::cli_result::ErrorKind::ConfigError;

    use super::*;

    #[test]
    fn test_clone_game_ok() {
        // implements drop and will be cleaned up
        let dir = tempdir().expect("Could not create directory");

        let result = clone_game(dir.path().to_str().expect("Could not extract path"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_config_ok() {
        let dir = tempdir().expect("Could not create directory");

        let cfg = Config {
            game_path: "test_path".to_string(),
        };
        let result = create_config(cfg, dir.path().to_str().expect("Could not get path"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_impl_ok() {
        let clone_game = |_: &str| -> CliResult<()> { Ok(()) };
        let create_config = |_: Config, _: &str| -> CliResult<()> { Ok(()) };
        let cfg_init = CfgInit {
            game_path: "".to_string(),
            config_path: "".to_string(),
        };

        let result = init_impl(cfg_init, clone_game, create_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_impl_clone_err() {
        testing_logger::setup();
        let error_to_return = CliError::new(GameCloneError, "message", Some("reason".to_string()));
        let clone_game = |_: &str| -> CliResult<()> { Err(error_to_return.clone()) };
        let create_config = |_: Config, _: &str| -> CliResult<()> { Ok(()) };
        let cfg_init = CfgInit {
            game_path: "".to_string(),
            config_path: "".to_string(),
        };

        let result = init_impl(cfg_init, clone_game, create_config);

        assert_eq!(result.err().expect("Expected error"), error_to_return);
    }

    #[test]
    fn test_init_impl_config_err() {
        let error_to_return = CliError::new(ConfigError, "message", Some("reason".to_string()));

        let clone_game = |_: &str| -> CliResult<()> { Ok(()) };
        let create_config = |_: Config, _: &str| -> CliResult<()> { Err(error_to_return.clone()) };
        let cfg_init = CfgInit {
            game_path: "".to_string(),
            config_path: "".to_string(),
        };

        let result = init_impl(cfg_init, clone_game, create_config);

        assert_eq!(result.err().expect("Expected error"), error_to_return);
    }
}
