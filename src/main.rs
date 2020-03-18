#[macro_use]
extern crate lazy_static;

mod cheat;
mod display;
mod filesystem;
mod flows;
mod fzf;
mod git;
mod handler;
mod option;
mod structures;
mod terminal;
mod welcome;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    handler::handle_config(option::config_from_env())
}
