#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod actor;
mod cheatsh;
mod clipboard;
mod env_var;
mod extractor;
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
mod ui;
mod url;
mod welcome;
mod writer;

pub use handler::handle_config;
pub use structures::config::{config_from_env, config_from_iter};
