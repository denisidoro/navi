pub mod core;
pub mod func;
pub mod info;
pub mod preview;
pub mod preview_var;
pub mod repo;
pub mod shell;

use crate::handler;
use crate::structures::config::Command::{Fn, Info, Preview, PreviewVar, Repo, Widget};
use crate::structures::config::{Config, RepoCommand};
use anyhow::Context;
use anyhow::Result;

pub fn handle_config(config: Config) -> Result<()> {
    match config.cmd.as_ref() {
        None => handler::core::main(config),

        Some(c) => match c {
            Preview { line } => handler::preview::main(&line),

            PreviewVar {
                selection,
                query,
                variable,
            } => handler::preview_var::main(&selection, &query, &variable),

            Widget { shell } => handler::shell::main(shell).context("Failed to print shell widget code"),

            Fn { func, args } => handler::func::main(func, args.to_vec())
                .with_context(|| format!("Failed to execute function `{:#?}`", func)),

            Info { info } => {
                handler::info::main(info).with_context(|| format!("Failed to fetch info `{:#?}`", info))
            }

            Repo { cmd } => match cmd {
                RepoCommand::Add { uri } => {
                    handler::repo::add(uri.clone(), &config.finder)
                        .with_context(|| format!("Failed to import cheatsheets from `{}`", uri))?;
                    handler::core::main(config)
                }
                RepoCommand::Browse => {
                    handler::repo::browse(&config.finder).context("Failed to browse featured cheatsheets")?;
                    handler::core::main(config)
                }
            },
        },
    }
}
