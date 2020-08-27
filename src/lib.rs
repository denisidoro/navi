#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod display;
mod fetcher;
mod finder;
mod flows;
mod handler;
mod parser;
mod structures;
mod tldr;
mod welcome;
mod common;
mod filesystem;

pub use handler::handle_config;
pub use structures::config::{config_from_env, config_from_iter};
pub use common::file_issue::FileAnIssue;
