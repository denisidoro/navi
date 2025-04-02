use crate::filesystem;
use crate::prelude::*;
use clap::{Args, Subcommand};

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub info: Info,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Info {
    /// Prints a cheatsheet example.
    CheatsExample,
    /// Prints a configuration file example.
    ConfigExample,

    /// [DEPRECATED] Prints the default cheatsheets path.
    /// Please use `info default-cheats-path` instead.
    CheatsPath,
    /// [DEPRECATED] Prints the default configuration path.
    /// Please use `info default-config-path` instead.
    ConfigPath,

    /// Prints the default cheatsheets path.
    DefaultCheatsPath,
    /// Prints the default configuration path.
    DefaultConfigPath,
}


impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let info = &self.info;

        match info {
            // Here should be the example commands
            Info::CheatsExample => println!("{}", include_str!("../../docs/cheat_example.cheat")),
            Info::ConfigExample => println!("{}", include_str!("../../docs/config_file_example.yaml")),

            // Here should be the old deprecated default value commands
            Info::CheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
            Info::ConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),

            // Here should be the default values (computed at compile time)
            Info::DefaultCheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
            Info::DefaultConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),
        }
        Ok(())
    }
}
