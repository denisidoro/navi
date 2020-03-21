use crate::flows;
use crate::flows::core::Variant;
use crate::structures::option::Command::{Best, Fn, Preview, Query, Repo, Search, Widget};
use crate::structures::option::{Config, RepoCommand};
use anyhow::Context;
use anyhow::Error;

pub fn handle_config(mut config: Config) -> Result<(), Error> {
    match config.cmd.as_mut() {
        None => Ok(flows::core::main(Variant::Core, config, true)
            .expect("TODO: convert this flow fn to anyhow error")),
        Some(c) => {
            match c {
                Preview { line } => flows::preview::main(&line[..]),
                Query { query } => Ok(flows::query::main(query.clone(), config)
                    .expect("TODO: convert this flow fn to anyhow error")),
                Best { query, args } => Ok(flows::best::main(query.clone(), args.to_vec(), config)
                    .expect("TODO: convert this flow fn to anyhow error")),
                Search { query } => Ok(flows::search::main(query.clone(), config)
                    .expect("TODO: convert this flow fn to anyhow error")),
                Widget { shell } => Ok(flows::shell::main(&shell[..])
                    .expect("TODO: convert this flow fn to anyhow error")),
                Fn { func, args } => flows::func::main(func.clone(), args.to_vec())
                    .with_context(|| format!("Failed to execute function {}", func)),
                Repo { cmd } => match cmd {
                    RepoCommand::Add { uri } => flows::repo::add(uri.clone())
                        .with_context(|| format!("Failed to import cheatsheets from {}", uri)),
                    RepoCommand::Browse => {
                        flows::repo::browse().context("Failed to browse featured cheatsheets")
                    }
                },
            }
        }
    }
}
