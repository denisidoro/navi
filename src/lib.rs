#[macro_use]
extern crate lazy_static;

mod display;
mod filesystem;
mod flows;
mod fzf;
mod git;
mod handler;
mod parser;
mod structures;
mod terminal;
mod welcome;

pub use handler::handle_config;
pub use structures::option::{config_from_env, config_from_iter};
