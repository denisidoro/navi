#[macro_use]
extern crate lazy_static;

use std::error::Error;

mod cheat;
mod cmds;
mod display;
mod filesystem;
mod fzf;
mod option;
mod terminal;

use crate::cmds::core::Variant;
use option::{InternalCommand, RepoCommand};
use option::Command::{Query, Best, Search, Widget, Fn, Repo};

fn main() -> Result<(), Box<dyn Error>> {
    match option::internal_command() {
        Some(InternalCommand::Preview { line }) => cmds::preview::main(line),
        _ => {
            let mut config = option::parse();
            match config.cmd.as_mut() {
                None => cmds::core::main(Variant::Core, config, true),
                Some(c) => match c {
                    Query { query } => cmds::query::main(query.clone(), config),
                    Best { query, args } => {
                        cmds::best::main(query.clone(), args.to_vec(), config)
                    }
                    Search { query } => cmds::search::main(query.clone(), config),
                    Widget { shell } => cmds::shell::main(&shell[..]),
                    Fn { func, args } => cmds::func::main(func.clone(), args.to_vec()),
                    Repo { cmd } => {
                        match cmd {
                            RepoCommand::Add { uri } => cmds::repo::add(uri.clone()),
                        }
                    }
                },
            }
        }
    }
}
