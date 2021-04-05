#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod actor;
mod cheatsh;
mod clipboard;
mod cmds;
mod display;
mod env_vars;
mod extractor;
mod fetcher;
mod filesystem;
mod finder;
mod fs;
mod git;
mod handler;
mod hash;
mod parser;
mod shell;
mod structures;
mod terminal;
mod tldr;
mod url;
mod welcome;

pub use handler::handle_config;
pub use structures::config::{config_from_env, config_from_iter};
