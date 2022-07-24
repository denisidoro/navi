use crate::prelude::*;
use clap::{Args, Subcommand};

pub mod add;
pub mod browse;

#[derive(Debug, Clone, Subcommand)]
pub enum RepoCommand {
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
    },
    /// Browses for featured cheatsheet repos
    Browse,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub cmd: RepoCommand,
}
