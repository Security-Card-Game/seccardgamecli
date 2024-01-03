use game_lib::file::general::ensure_directory_exists;
use git2::Repository;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

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

pub fn init(init: CfgInit) {
    init_impl(init, clone_game, create_config)
}

fn init_impl<F, G>(init: CfgInit, clone_game: F, create_config: G)
where
    F: Fn(&str) -> Result<(), Error>,
    G: Fn(Config, &str) -> Result<(), Error>,
{
    match clone_game(init.game_path.as_str()) {
        Ok(_) => info!("Downloaded game into {}", init.game_path),
        Err(e) => error!("{}", e),
    }
    match create_config(
        Config {
            game_path: init.game_path,
        },
        init.config_path.as_str(),
    ) {
        Ok(_) => info!("Config file created"),
        Err(e) => error!("{}", e),
    }
}

pub fn read_config(path_to_config: &str) -> Config {
    match fs::read_to_string(path_to_config) {
        Ok(cfg) => serde_json::from_str(cfg.as_str())
            .expect("Could not parse config file, did you run init?"),
        Err(e) => panic!("Could not read config file {}", e.to_string()),
    }
}

fn create_config(cfg: Config, path_to_config: &str) -> std::io::Result<()> {
    let path = Path::new(path_to_config);
    let mut path_to_write = path.to_path_buf();
    if path.is_dir() {
        path_to_write.push(DEFAULT_CFG);
    } else {
        if let Some(dir) = path.parent() {
            match ensure_directory_exists(dir.to_str().unwrap().trim()) {
                Ok(_) => (),
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Could not create directory {}", path.to_str().unwrap_or("")),
                    ))
                }
            }
        }
    };
    match serde_json::to_string_pretty(&cfg) {
        Ok(json) => write_config(json, path_to_write),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not serialize config",
        )),
    }
}

fn write_config(json: String, path: PathBuf) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())
}

fn clone_game(path: &str) -> std::io::Result<()> {
    match ensure_directory_exists(path) {
        Ok(_) => (),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Could not create directory {}", path),
            ))
        }
    };
    info!("Cloning game repository...");
    match Repository::clone(GAME_REPO, path) {
        Ok(_) => (),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to clone game: {}", e),
            ))
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;
    use std::io::ErrorKind;
    use std::sync::{Arc, Mutex, Once};
    use tempfile::tempdir;

    static INIT: Once = Once::new();
    #[test]
    fn test_clone_game_ok() {
        // implements drop and will be cleaned up
        let dir = tempdir().expect("Could not create directory");

        let result = clone_game(dir.path().to_str().expect("Could not extract path"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_clone_game_err() {
        // This directory should not exist
        let result = clone_game("/invalid/path/");
        assert!(result.is_err());
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
    fn test_create_config_err() {
        // This directory should not exist
        let cfg = Config {
            game_path: "test_path".to_string(),
        };
        let result = create_config(cfg, "/invalid/path/");
        assert!(result.is_err());
    }

    #[test]
    fn test_init_impl_ok() {
        testing_logger::setup();
        let clone_game = |_: &str| -> Result<(), Error> { Ok(()) };
        let create_config = |_: Config, _: &str| -> Result<(), Error> { Ok(()) };
        let cfg_init = CfgInit {
            game_path: "".to_string(),
            config_path: "".to_string(),
        };

        init_impl(cfg_init, clone_game, create_config);

        validate_error_log_count(0);
    }

    /// Test for `init_impl` function when `clone_game` returns an error.
    ///
    /// This test case verifies that when `clone_game` returns an error, `init_impl` logs an error message.
    /// It sets up a logger using `Logger::with` and `TestLogWriter`, and then calls `init_impl` with a mock implementation of `clone_game` that always returns
    #[test]
    fn test_init_impl_clone_err() {
        testing_logger::setup();
        let clone_game =
            |_: &str| -> Result<(), Error> { Err(Error::new(ErrorKind::Other, "error")) };
        let create_config = |_: Config, _: &str| -> Result<(), Error> { Ok(()) };
        let cfg_init = CfgInit {
            game_path: "".to_string(),
            config_path: "".to_string(),
        };

        init_impl(cfg_init, clone_game, create_config);

        validate_error_log_count(1);
    }

    fn validate_error_log_count(count: usize) {
        testing_logger::validate(|log| {
            let error_logs: Vec<_> = log.iter().filter(|&l| l.level == Level::Error).collect();
            assert_eq!(error_logs.len(), count);
        });
    }

    /// Test for `init_impl` function when `create_config` returns an error.
    ///
    /// This test case verifies that when `create_config` returns an error, `init_impl` logs an error message.
    /// It sets up a logger using `Logger::with` and `TestLogWriter`, and then calls `init_impl` with a mock implementation of `clone_game` that always returns
    #[test]
    fn test_init_impl_config_err() {
        testing_logger::setup();
        let clone_game = |_: &str| -> Result<(), Error> { Ok(()) };
        let create_config = |_: Config, _: &str| -> Result<(), Error> {
            Err(Error::new(ErrorKind::Other, "error"))
        };
        let cfg_init = CfgInit {
            game_path: "".to_string(),
            config_path: "".to_string(),
        };

        init_impl(cfg_init, clone_game, create_config);

        validate_error_log_count(1);
    }
}
