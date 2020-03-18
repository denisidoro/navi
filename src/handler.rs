use crate::cmds;
use crate::cmds::core::Variant;
use crate::option::Command::{Best, Fn, Preview, Query, Repo, Search, Widget};
use crate::option::{Config, RepoCommand};
use std::error::Error;

pub fn handle_config(mut config: Config) -> Result<(), Box<dyn Error>> {
    match config.cmd.as_mut() {
        None => cmds::core::main(Variant::Core, config, true),
        Some(c) => match c {
            Preview { line } => cmds::preview::main(&line[..]),
            Query { query } => cmds::query::main(query.clone(), config),
            Best { query, args } => cmds::best::main(query.clone(), args.to_vec(), config),
            Search { query } => cmds::search::main(query.clone(), config),
            Widget { shell } => cmds::shell::main(&shell[..]),
            Fn { func, args } => cmds::func::main(func.clone(), args.to_vec()),
            Repo { cmd } => match cmd {
                RepoCommand::Add { uri } => cmds::repo::add(uri.clone()),
                RepoCommand::Browse => cmds::repo::browse(),
            },
        },
    }
}
