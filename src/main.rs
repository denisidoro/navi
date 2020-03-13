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
use option::{Command, InternalCommand};

fn main() -> Result<(), Box<dyn Error>> {
    match option::internal_command() {
        Some(InternalCommand::Preview { line }) => cmds::preview::main(line),
        _ => {
            let mut config = option::parse();
            match config.cmd.as_mut() {
                None => cmds::core::main(Variant::Core, config, true),
                Some(c) => match c {
                    Command::Query { query } => cmds::query::main(query.clone(), config),
                    Command::Best { query, args } => {
                        cmds::best::main(query.clone(), args.to_vec(), config)
                    }
                    Command::Search { query } => cmds::search::main(query.clone(), config),
                    Command::Widget { shell } => cmds::shell::main(&shell[..]),
                    Command::Fn { func, args } => cmds::func::main(func.clone(), args.to_vec()),
                    Command::Home => cmds::home::main(),
                },
            }
        }
    }
}
