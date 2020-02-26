use std::error::Error;

mod option;
mod shell;
mod preview;
mod fzf;
mod parse;
mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = option::parse();

    match matches.subcommand().0 {
        "preview" => preview::main(&matches),
        "widget" => shell::main(&matches),
        _ => core::main(&matches),
    }
}