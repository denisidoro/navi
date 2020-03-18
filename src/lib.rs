#[macro_use]
extern crate lazy_static;

mod display;
mod filesystem;
mod flows;
mod fzf;
mod git;
mod parser;
mod structures;
mod terminal;
mod welcome;
mod handler;

pub use structures::option::{config_from_env, config_from_iter};
pub use handler::handle_config;