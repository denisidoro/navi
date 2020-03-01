// #[macro_use]
// extern crate lazy_static;

use std::env;
use std::error::Error;

mod cheat;
mod cmds;
mod display;
mod filesystem;
mod fzf;
mod option;

use crate::cmds::core::Variant;
use option::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    if args.next() == Some(String::from("preview")) {
        cmds::preview::main(args.next().unwrap())
    } else {
        let mut config = option::parse();

        match config.cmd.as_mut() {
            Some(c) => match c {
                Command::Query { args } => cmds::query::main(args.to_vec(), config),
                Command::Best { args } => cmds::best::main(args.to_vec(), config),
                Command::Search { args } => cmds::search::main(args.to_vec(), config),
                Command::Widget { shell } => cmds::shell::main(&shell[..]),
                Command::Home => cmds::home::main(),
            },
            None => cmds::core::main(Variant::Core, config),
        }
    }
}
