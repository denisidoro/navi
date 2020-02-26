use std::error::Error;

mod core;
mod filesystem;
mod fzf;
mod option;
mod parse;
mod preview;
mod shell;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = option::parse();

    match matches.subcommand().0 {
        "preview" => preview::main(&matches),
        "widget" => shell::main(&matches),
        _ => core::main(&matches),
    }
}
