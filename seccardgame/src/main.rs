mod cards;
mod cli;

use crate::cli::config::{init, CfgInit, Config};
use clap::{Arg, Command};
use std::process::exit;
use flexi_logger::Logger;

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
}

fn main() {
    Logger::try_with_env_or_str("info").expect("Logger to be initialized")
        .start().expect("Logger to be started)");

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
        _ => exit(-1),
    }
}

fn load_config(cfg: String) -> Config {
    cli::config::Config::read_config(cfg.as_str())
}
