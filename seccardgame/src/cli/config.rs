use game_lib::file::general::ensure_directory_exists;
use game_lib::print_to_stderr;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{fs, io};
use std::io::Write;
use std::path::{Path, PathBuf};

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
    match clone_game(init.game_path.as_str()) {
        Ok(_) => println!("Downloaded game into {}", init.game_path),
        Err(e) => print_to_stderr(e.to_string().as_str()),
    }
    match create_config(
        Config {
            game_path: init.game_path,
        },
        init.config_path.as_str(),
    ) {
        Ok(_) => println!("Config file created"),
        Err(e) => print_to_stderr(e.to_string().as_str()),
    }
}

pub fn read_config(path_to_config: &str) -> Config {
    match fs::read_to_string(path_to_config) {
        Ok(cfg) => serde_json::from_str(cfg.as_str()).expect("Could not parse config file, did you run init?"),
        Err(e) => panic!("Could not read config file {}", e.to_string())
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
    println!("Cloning game repository...");
    match Repository::clone(GAME_REPO, path) {
        Ok(_) => (),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to clone game: {}", e),
            ))
        }
    };
    println!("Clones game repository into '{}'", path);
    Ok(())
}
