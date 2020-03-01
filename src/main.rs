// #[macro_use]
// extern crate lazy_static;

use std::error::Error;

mod cheat;
mod cmds;
mod display;
mod filesystem;
mod fzf;
mod option;

use crate::cmds::core::Variant;
use option::{Command, InternalCommand};

fn main() -> Result<(), Box<dyn Error>> {
    match option::internal_command() {
        Some(InternalCommand::Preview { line }) => cmds::preview::main(line),
        _ => {
            let mut config = option::parse();
            match config.cmd.as_mut() {
                None => cmds::core::main(Variant::Core, config),
                Some(c) => match c {
                    Command::Query { args } => cmds::query::main(args.to_vec(), config),
                    Command::Best { args } => cmds::best::main(args.to_vec(), config),
                    Command::Search { args } => cmds::search::main(args.to_vec(), config),
                    Command::Widget { shell } => cmds::shell::main(&shell[..]),
                    Command::Home => cmds::home::main(),
                },
            }
        }
    }
}
