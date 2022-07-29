#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate anyhow;

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

pub use commands::handle;
