mod cards;
mod cli;
mod game;

use crate::cli::config::{init, CfgInit, Config};
use clap::{Arg, Command};
use std::process::exit;
use flexi_logger::Logger;
use log::error;
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::game::create::create_game;

fn cli() -> Command {
    Command::new("seccardgame")
        .about("Seccardgame CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("Path to config file, defaults to secgame_cfg.json")
                .default_value("secgame_cfg.json"),
        )
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initializes the cards and writes config")
                .arg_required_else_help(false)
                .arg(Arg::new("path").default_missing_value("game")),
        )
        .subcommand(
            Command::new("cards")
                .about("Operate on cards")
                .subcommand_required(true)
                .subcommand(Command::new("create").about("Create a card").arg_required_else_help(false))
                .subcommand(Command::new("stats").about("Prints stats")),
        )
        .subcommand(
            Command::new("game")
                .about("Operate on games")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Creates a deck to play a game")
                        .arg_required_else_help(false)
                        .arg(Arg::new("path").default_missing_value("deck")))
                .subcommand(Command::new("play")
                    .about("Plays a game with a deck")
                    .arg_required_else_help(false)
                    .arg(Arg::new("path").default_missing_value("deck")))
                )
}

fn main() {
    Logger::try_with_env_or_str("info").expect("Logger to be initialized")
        .start().expect("Logger to be started)");
    match handle_commands() {
        Ok(_) => exit(0),
        Err(e) => {
            error!("{}", e);
            exit(1)
        }
    }
}

fn handle_commands() -> CliResult<()> {

    let matches = cli().get_matches();
    let cfg = matches.get_one::<String>("config").unwrap().clone();
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let path = if let Some(game_path) = sub_matches.get_one::<String>("path") {
                game_path.clone()
            } else {
                "game".to_string()
            };
            let cfg_init = CfgInit {
                game_path: path,
                config_path: cfg,
            };
            init(cfg_init)
        }
        Some(("cards", sub_matches)) => {
            let config = load_config(cfg);
            match sub_matches.subcommand() {
                Some(("create", _)) => cards::crud::create(&config),
                Some(("stats", _)) => cards::stats::print_stats(&config),
                _ => exit(-1),
            }
        },
        Some(("game", sub_matches)) => {
            let config = load_config(cfg);
            match sub_matches.subcommand() {
                Some(("create", sub_matches)) => {
                    let path = if let Some(deck_path) = sub_matches.get_one::<String>("path") {
                        deck_path.clone()
                    } else {
                        "deck".to_string()
                    };
                    create_game(path, &config)                },
                Some(("play", _)) => Err(CliError { kind: ErrorKind::NotImplemented, message: "not yet done".to_string(), original_message: None }),
                _ => exit(-1),
            }
        },
        _ => exit(-1),
    }
}

fn load_config(cfg: String) -> Config {
    cli::config::Config::read_config(cfg.as_str())
}
