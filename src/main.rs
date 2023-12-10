mod file;
mod history;
mod paths;
mod utils;

use clap::{Arg, ArgMatches, Command};
use paths::{check_file_exists_else_create, ProdPaths};
use std::process::exit;
use utils::ProdUtilities;

use crate::file::File;

fn register_commands() -> ArgMatches {
    let history_cmd = Command::new("history").about("Show the history of files trashed");

    let undo_cmd = Command::new("undo").about("Put the last file in the trash back where it was");

    let rm_cmd = Command::new("rm")
        .arg(Arg::new("path").num_args(0..))
        .arg_required_else_help(true)
        .about("Move a file to the trash");

    Command::new("bin_chicken")
        .subcommand(rm_cmd)
        .subcommand(undo_cmd)
        .subcommand(history_cmd)
        .arg_required_else_help(true)
        .get_matches()
}

fn main() {
    check_file_exists_else_create::<ProdPaths>();
    let cmd = register_commands();

    match cmd.subcommand() {
        Some(("history", _)) => {
            println!("hello from history");
            exit(0)
        }
        Some(("rm", path)) => {
            let test: Vec<_> = path.get_many::<String>("path").unwrap().collect();
            // Create as many files as needed
            // each file should move itself to the trash
            // and update the history file
            File::<ProdPaths, ProdUtilities>::new_from_name(test[0]).move_to_trash();
            exit(0)
        }
        Some(("undo", _)) => {
            File::<ProdPaths, ProdUtilities>::new_from_history().restore_from_trash();
            exit(0)
        }
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
