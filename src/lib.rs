#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate anyhow;

mod clients;
mod commands;
mod common;
mod config;
mod env_var;
mod filesystem;
mod finder;
mod parser;
mod prelude;
mod serializer;
mod structures;
mod terminal;
mod welcome;

pub use commands::handle;
