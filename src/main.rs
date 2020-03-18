#[macro_use]
extern crate lazy_static;

mod cheat;
mod cmds;
mod display;
mod filesystem;
mod fnv;
mod fzf;
mod git;
mod handler;
mod option;
mod terminal;
mod welcome;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
        handler::handle_config(option::config_from_env())
}
