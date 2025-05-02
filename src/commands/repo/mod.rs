use crate::commands;
use crate::prelude::*;
use clap::{Args, Subcommand};

pub mod add;
pub mod browse;
mod sync;
mod list;

#[derive(Debug, Clone, Subcommand)]
pub enum RepoCommand {
    /// Browses for featured cheatsheet repos
    Browse,
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
    },
    /// Synchronize either all cheatsheet repositories or a given one.
    Sync {
        /// The name of the cheatsheet repository to sync.
        name: Option<String>
    },
    /// List all downloaded repositories
    List
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub cmd: RepoCommand,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        match &self.cmd {
            RepoCommand::Add { uri } => {
                add::main(uri.clone())
                    .with_context(|| format!("Failed to import cheatsheets from `{uri}`"))?;

                commands::core::main()
            }
            RepoCommand::Browse => {
                let repo = browse::main().context("Failed to browse featured cheatsheets")?;
                add::main(repo.clone())
                    .with_context(|| format!("Failed to import cheatsheets from `{repo}`"))?;

                commands::core::main()
            }
            RepoCommand::Sync { name } => {
                sync::main(name.clone())
                    // TODO: Remove the debug extension later on
                    .with_context(|| format!("Failed to synchronize cheatsheets from `{:?}`", name))
            }
            RepoCommand::List => {
                Ok(list::main())
            }
        }
    }
}
