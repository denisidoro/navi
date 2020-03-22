use crate::flows;
use crate::flows::core::Variant;
use crate::structures::option::Command::{Best, Fn, Preview, Query, Repo, Search, Widget};
use crate::structures::option::{Config, RepoCommand};
use anyhow::Context;
use anyhow::Error;

pub fn handle_config(mut config: Config) -> Result<(), Error> {
    match config.cmd.as_mut() {
        None => flows::core::main(Variant::Core, config, true),

        Some(c) => match c {
            Preview { line } => flows::preview::main(&line[..]),

            Query { query } => {
                let error_string = format!("Failed to filter cheatsheets for {}", &query);
                flows::query::main(query.clone(), config).context(error_string)
            }

            Best { query, args } => {
                let error_string = format!("Failed to execute snippet similar to {}", &query);
                flows::best::main(query.clone(), args.to_vec(), config).context(error_string)
            }

            Search { query } => flows::search::main(query.clone(), config)
                .context("Failed to search for online cheatsheets"),

            Widget { shell } => flows::shell::main(&shell[..]),

            Fn { func, args } => flows::func::main(func.clone(), args.to_vec())
                .with_context(|| format!("Failed to execute function {}", func)),

            Repo { cmd } => match cmd {
                RepoCommand::Add { uri } => flows::repo::add(uri.clone())
                    .with_context(|| format!("Failed to import cheatsheets from {}", uri)),
                RepoCommand::Browse => {
                    flows::repo::browse().context("Failed to browse featured cheatsheets")
                }
            },
        },
    }
}
