use crate::filesystem;
use crate::prelude::*;
use clap::Args;
use clap::ValueEnum;
use crossterm::style::Stylize;

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[arg(ignore_case = true)]
    pub info: Info,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Info {
    CheatsExample,
    ConfigExample,

    CheatsPath,
    ConfigPath,

    DefaultCheatsPath,
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
            Info::CheatsPath => println!(
                "{} Please use `info default-cheats-path` instead.\n\n\
                {}",
                "DEPRECATED:".red(),
                &filesystem::default_cheat_pathbuf()?.to_string()
            ),
            Info::ConfigPath => println!(
                "{} Please use `info default-config-path` instead.\n\n\
                {}",
                "DEPRECATED:".red(),
                &filesystem::default_config_pathbuf()?.to_string()
            ),

            // Here should be the default values (computed at compile time)
            Info::DefaultCheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
            Info::DefaultConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),
        }
        Ok(())
    }
}
