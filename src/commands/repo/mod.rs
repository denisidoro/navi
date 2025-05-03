use crate::commands;
use crate::prelude::*;
use clap::{Args, Subcommand};

pub mod add;
pub mod browse;
mod list;
mod sync;

#[derive(Debug, Clone, Subcommand)]
pub enum RepoCommand {
    /// Browses for featured cheatsheet repos
    Browse,
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
        /// Assumes yes for all confirmations
        #[clap(short = 'y', long = "yes")]
        yes_flag: bool,
    },
    /// Synchronize either all cheatsheet repositories or a given one.
    Sync {
        /// The name of the cheatsheet repository to sync.
        name: Option<String>,
    },
    /// List all downloaded repositories
    List,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub cmd: RepoCommand,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        match &self.cmd {
            RepoCommand::Add { uri, yes_flag } => {
                add::main(uri.clone(), *yes_flag)
                    .with_context(|| format!("Failed to import cheatsheets from `{uri}`"))?;

                commands::core::main()
            }
            RepoCommand::Browse => {
                let repo = browse::main().context("Failed to browse featured cheatsheets")?;
                add::main(repo.clone(), false)
                    .with_context(|| format!("Failed to import cheatsheets from `{repo}`"))?;

                commands::core::main()
            }
            RepoCommand::Sync { name } => {
                sync::main(name.clone())
                    // TODO: Remove the debug extension later on
                    .with_context(|| format!("Failed to synchronize cheatsheets from `{:?}`", name))
            }
            RepoCommand::List => {
                list::main();

                Ok(())
            }
        }
    }
}
