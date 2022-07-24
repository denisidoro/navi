#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate anyhow;

mod actor;
mod cheat_variable;
mod cheatsh;
mod clipboard;
mod common;
mod config;
mod env_var;
mod extractor;
mod filesystem;
mod finder;
mod fs;
mod git;
mod handler;
mod hash;
mod parser;
mod prelude;
mod shell;
mod structures;
mod terminal;
mod tldr;
mod ui;
mod url;
mod welcome;
mod writer;

pub use handler::handle;
