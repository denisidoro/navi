use crate::cmds;
use crate::structures::config::Command::{Fn, Info, Preview, PreviewVar, Repo, Widget};
use crate::structures::config::{Config, RepoCommand};
use anyhow::Context;
use anyhow::Result;

pub fn handle_config(config: Config) -> Result<()> {
    match config.cmd.as_ref() {
        None => cmds::core::main(config),

        Some(c) => match c {
            Preview { line } => cmds::preview::main(&line),

            PreviewVar {
                selection,
                query,
                variable,
            } => cmds::preview::main_var(&selection, &query, &variable),

            Widget { shell } => cmds::shell::main(shell).context("Failed to print shell widget code"),

            Fn { func, args } => cmds::func::main(func, args.to_vec())
                .with_context(|| format!("Failed to execute function `{:#?}`", func)),

            Info { info } => {
                cmds::info::main(info).with_context(|| format!("Failed to fetch info `{:#?}`", info))
            }

            Repo { cmd } => match cmd {
                RepoCommand::Add { uri } => {
                    cmds::repo::add(uri.clone(), &config.finder)
                        .with_context(|| format!("Failed to import cheatsheets from `{}`", uri))?;
                    cmds::core::main(config)
                }
                RepoCommand::Browse => {
                    cmds::repo::browse(&config.finder).context("Failed to browse featured cheatsheets")?;
                    cmds::core::main(config)
                }
            },
        },
    }
}
