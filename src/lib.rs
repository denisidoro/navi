#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod clipboard;
mod display;
mod filesystem;
mod finder;
mod flows;
mod git;
mod handler;
mod parser;
mod structures;
mod terminal_width;
mod url;
mod welcome;
mod tldr;

pub use handler::handle_config;
pub use structures::config::{config_from_env, config_from_iter};
pub use structures::error::file_issue::FileAnIssue;
