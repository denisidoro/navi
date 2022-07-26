#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate anyhow;

mod actor;
mod cheat_variable;
mod clients;
mod clipboard;
mod commands;
mod common;
mod config;
mod env_var;
mod extractor;
mod filesystem;
mod finder;
mod fs;
mod git;
mod hash;
mod parser;
mod prelude;
mod serializer;
mod shell;
mod structures;
mod terminal;
mod url;
mod welcome;

pub use commands::handle;
