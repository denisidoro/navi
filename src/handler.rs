use crate::cmds;
use crate::structures::config::Command::{Alfred, Best, Fn, Preview, Query, Repo, Tldr, Widget};
use crate::structures::config::{AlfredCommand, Config, RepoCommand};
use anyhow::Context;
use anyhow::Error;

pub fn handle_config(config: Config) -> Result<(), Error> {
    match config.cmd.as_ref() {
        None => cmds::core::main(config),

        Some(c) => {
            match c {
                Preview { line } => cmds::preview::main(&line[..]),

                Widget { shell } => {
                    cmds::shell::main(&shell).context("Failed to print shell widget code")
                }

                Fn { func, args } => cmds::func::main(func.clone(), args.to_vec())
                    .with_context(|| format!("Failed to execute function `{}`", func)),

                Repo { cmd } => match cmd {
                    RepoCommand::Add { uri } => cmds::repo::add(uri.clone(), &config.finder)
                        .with_context(|| format!("Failed to import cheatsheets from `{}`", uri)),
                    RepoCommand::Browse => cmds::repo::browse(&config.finder)
                        .context("Failed to browse featured cheatsheets"),
                },

                Alfred { cmd } => match cmd {
                    AlfredCommand::Start => cmds::alfred::main(config)
                        .context("Failed to call Alfred starting function"),
                    AlfredCommand::Suggestions => cmds::alfred::suggestions(config, false)
                        .context("Failed to call Alfred suggestion function"),
                    AlfredCommand::Check => cmds::alfred::suggestions(config, true)
                        .context("Failed to call Alfred check function"),
                    AlfredCommand::Transform => cmds::alfred::transform()
                        .context("Failed to call Alfred transform function"),
                },

                _ => cmds::core::main(config),
            }
        }
    }
}
