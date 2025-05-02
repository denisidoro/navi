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
pub mod prelude;
mod structures;
mod welcome;

mod libs {
    pub mod dns_common;
    pub mod terminal;
}

pub use {commands::handle, filesystem::default_config_pathbuf};
