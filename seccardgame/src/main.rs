mod cli;
mod cards;

use std::process::exit;
use clap::Command;

fn cli() -> Command {
    Command::new("seccardgame")
        .about("Seccardgame CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("cards")
                .about("Operate on cards")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .arg_required_else_help(false)
                )
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("cards", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("create", _)) => {
                    cards::crud::create()
                }
                _ => exit(-1),
            }
        }
        _ => exit(0),
    }

}