mod cards;
mod cli;

use crate::cli::config::{init, CfgInit};
use clap::{Arg, Command};
use std::process::exit;

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
                .subcommand(Command::new("create").arg_required_else_help(false)),
        )
}

fn main() {
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
        Some(("cards", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", _)) => cards::crud::create(),
            _ => exit(-1),
        },
        _ => exit(0),
    }
}
