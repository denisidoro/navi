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
    CheatsPath,
    ConfigPath,
    ConfigExample,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let info = &self.info;

        match info {
            Info::CheatsExample => {
                println!("{}", include_str!("../../docs/examples/cheatsheet/example.cheat"))
            }
            Info::CheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
            Info::ConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),
            Info::ConfigExample => println!(
                "{}",
                include_str!("../../docs/examples/configuration/config-example.yaml")
            ),
        }
        Ok(())
    }
}
