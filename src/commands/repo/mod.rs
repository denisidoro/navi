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
    /// Imports cheatsheets from a git repository
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
        /// Assumes yes for all confirmations
        #[clap(short = 'y', long = "yes")]
        yes_flag: bool,

        /// Lets you target a specific git ref (e.g. a branch), anything accepted by the `--branch` parameter of `git-clone`
        #[clap(short = 'b', long = "branch")]
        branch: Option<String>,
        /// Import all cheatsheets from repo without prompting
        #[clap(short = 'a', long, visible_short_alias = 'y', visible_alias = "yes")]
        all: bool,
    },
    /// Synchronize either all cheatsheet repositories or a given one.
    Sync {
        /// The name of the cheatsheet repository to sync.
        name: Option<String>,
    },
    /// List all downloaded repositories
    List,
    /// Browses for featured cheatsheet repos
    Browse {
        /// Import all cheatsheets from selected repo without prompting
        #[clap(short = 'a', long, visible_short_alias = 'y', visible_alias = "yes")]
        all: bool,
    },
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub cmd: RepoCommand,
}

pub const HELP_NO_REPOSITORIES_FOUND: &str = r#"
Uh Oh! It seems you haven't downloaded a cheatsheet repository yet.
What you can do:

- `navi repo add` to add a cheatsheet repository
- `navi repo browse` to browse recommended cheatsheet repositories
"#;

pub const HELP_NO_GIVEN_REPOSITORIES_FOUND: &str = r#"
Uh Oh! It seems I can't find the repository...
Are you sure you downloaded it?

What you can do:

- `navi repo list` to list the current cheatsheets installed locally
- `navi repo add` to add a cheatsheet repository
"#;

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        match &self.cmd {
            RepoCommand::Add {
                uri,
                branch,
                all,
                yes_flag,
            } => {
                add::main(uri.clone(), branch, *all, *yes_flag)
                    .with_context(|| format!("Failed to import cheatsheets from `{uri}`"))?;

                commands::core::main()
            }
            RepoCommand::Browse { all } => {
                let repo = browse::main().context("Failed to browse featured cheatsheets")?;
                add::main(repo.clone(), *all, false, &None)
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
