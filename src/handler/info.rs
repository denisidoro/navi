use clap::Args;

use crate::filesystem;
use crate::prelude::*;

const INFO_POSSIBLE_VALUES: &[&str] = &["cheats-example", "cheats-path", "config-path", "config-example"];

impl FromStr for Info {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cheats-example" => Ok(Info::CheatsExample),
            "cheats-path" => Ok(Info::CheatsPath),
            "config-example" => Ok(Info::ConfigExample),
            "config-path" => Ok(Info::ConfigPath),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(possible_values = INFO_POSSIBLE_VALUES, ignore_case = true)]
    pub info: Info,
}

#[derive(Debug, Clone)]
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
            Info::CheatsExample => println!("{}", include_str!("../../docs/cheat_example.cheat")),
            Info::CheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
            Info::ConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),
            Info::ConfigExample => println!("{}", include_str!("../../docs/config_file_example.yaml")),
        }
        Ok(())
    }
}
