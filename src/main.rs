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
        let mut config = option::parse();

        println!("{:#?}", config.path);

        match config.cmd.as_mut() {
            Some(c) => match c {
                Command::Query { args } => cmds::query::main(args.to_vec(), config),
                Command::Best { args } => cmds::best::main(args.to_vec(), config),
                Command::Search { args } => cmds::search::main(args.to_vec(), config),
                Command::Widget { shell } => cmds::shell::main(&shell[..]),
            },
            None => cmds::core::main("", config),
        }
    }
}
