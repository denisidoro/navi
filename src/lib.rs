#[macro_use]
extern crate lazy_static;

mod clients;
mod commands;
mod common;
mod config;
mod deser;
mod env_var;
mod filesystem;
mod finder;
mod parser;
mod prelude;
mod structures;
mod welcome;

pub use {commands::handle, filesystem::default_config_pathbuf};
