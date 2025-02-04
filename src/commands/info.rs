use clap::Args;
use clap::ValueEnum;

use crate::filesystem;
use crate::prelude::*;

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[arg(ignore_case = true)]
    pub info: Info,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Info {
    CheatsExample,
    ConfigExample,

    DefaultCheatsPath,
    DefaultConfigPath,

    CheatsPath,
    ConfigPath,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let info = &self.info;

        match info {
            // Here should be the example files
            Info::CheatsExample => println!("{}", include_str!("../../docs/cheat_example.cheat")),
            Info::ConfigExample => println!("{}", include_str!("../../docs/config_file_example.yaml")),

            // Here should be the default values (computed at compile time)
            Info::DefaultCheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
            Info::DefaultConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),

            // Here should be the current values (computed at execution time)
            Info::ConfigPath => println!("{}", &filesystem::current_config_pathbuf()?.to_string()),
            Info::CheatsPath => println!("{}", &filesystem::current_cheat_pathbuf()?.to_string()),
        }
        Ok(())
    }
}
