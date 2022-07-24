use clap::Args;

use crate::prelude::*;
use crate::shell::Shell;

const WIDGET_POSSIBLE_VALUES: &[&str] = &["bash", "zsh", "fish", "elvish"];

impl FromStr for Shell {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "fish" => Ok(Shell::Fish),
            "elvish" => Ok(Shell::Elvish),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(possible_values = WIDGET_POSSIBLE_VALUES, ignore_case = true, default_value = "bash")]
    pub shell: Shell,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let shell = &self.shell;

        let content = match shell {
            Shell::Bash => include_str!("../../shell/navi.plugin.bash"),
            Shell::Zsh => include_str!("../../shell/navi.plugin.zsh"),
            Shell::Fish => include_str!("../../shell/navi.plugin.fish"),
            Shell::Elvish => include_str!("../../shell/navi.plugin.elv"),
        };

        println!("{}", content);

        Ok(())
    }
}
