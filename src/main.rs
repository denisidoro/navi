use std::env;
use std::error::Error;

mod cmds;
mod filesystem;
mod fzf;
mod option;
mod parse;

use option::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    if args.next() == Some(String::from("preview")) {
        cmds::preview::main(args.next().unwrap())
    } else {
        let config = option::parse();

        match config.cmd {
            Some(c) => match c {
                Command::Query { args } => cmds::query::main(args),
                Command::Best { args } => cmds::best::main(args),
                Command::Search { args } => cmds::search::main(args),
                Command::Widget { shell } => cmds::shell::main(&shell[..]),
            },
            None => cmds::core::main(config),
        }

        /* match matches.subcommand().0 {
            "preview" => cmds::preview::main(&matches),
            "widget" => cmds::shell::main(&matches),
            "best" => cmds::best::main(&matches),
            "search" => cmds::search::main(&matches),
            "query" => cmds::query::main(&matches),
            _ => cmds::core::main(&matches),
        }*/
    }
}
