#[macro_use]
extern crate lazy_static;

mod cheat;
mod cmds;
mod display;
mod filesystem;
mod fzf;
mod option;
mod terminal;
mod handler;
mod welcome;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let internal_cmd = option::internal_command_from_env();
    if let Some(cmd) = internal_cmd {
        handler::handle_internal_command(cmd)
    } else {
        handler::handle_config(option::config_from_env())
    }
}
