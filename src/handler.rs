use crate::flows;
use crate::flows::core::Variant;
use crate::structures::option::Command::{Alfred, Best, Fn, Preview, Query, Repo, Search, Widget};
use crate::structures::option::{AlfredCommand, Config, RepoCommand};
use anyhow::Context;
use anyhow::Error;

pub fn handle_config(config: Config) -> Result<(), Error> {
    match config.cmd.as_ref() {
        None => flows::core::main(Variant::Core, config, true),

        Some(c) => match c {
            Preview { line } => flows::preview::main(&line[..]),

            Query { query } => {
                let query_clone = query.clone();
                flows::query::main(query.clone(), config)
                    .with_context(|| format!("Failed to filter cheatsheets for {}", query_clone))
            }

            Best { query, args } => {
                let query_clone = query.clone();
                flows::best::main(query.clone(), args.to_vec(), config).with_context(|| {
                    format!("Failed to execute snippet similar to {}", query_clone)
                })
            }

            Search { query } => flows::search::main(query.clone(), config)
                .context("Failed to search for online cheatsheets"),

            Widget { shell } => flows::shell::main(&shell[..]),

            Fn { func, args } => flows::func::main(func.clone(), args.to_vec())
                .with_context(|| format!("Failed to execute function `{}`", func)),

            Repo { cmd } => match cmd {
                RepoCommand::Add { uri } => flows::repo::add(uri.clone(), &config.finder)
                    .with_context(|| format!("Failed to import cheatsheets from `{}`", uri)),
                RepoCommand::Browse => flows::repo::browse(&config.finder)
                    .context("Failed to browse featured cheatsheets"),
            },

            Alfred { cmd } => match cmd {
                AlfredCommand::Start => flows::alfred::main(config),
                AlfredCommand::Suggestions => flows::alfred::suggestions(config),
                AlfredCommand::Transform => flows::alfred::transform(),
            },
        },
    }
}
