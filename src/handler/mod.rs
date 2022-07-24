pub mod core;
pub mod func;
pub mod info;
pub mod preview;
pub mod repo;
pub mod shell;

#[cfg(not(feature = "disable-repo-management"))]
use crate::config::Command::Repo;
use crate::config::Command::{Fn, Info, Preview, PreviewVar, PreviewVarStdin, Widget};
use crate::handler;
use crate::prelude::*;
use repo::RepoCommand;

pub fn handle() -> Result<()> {
    match CONFIG.cmd() {
        None => handler::core::main(),

        Some(c) => match c {
            Preview(input) => handler::preview::main(&input.line),

            PreviewVarStdin => handler::preview::var_stdin::main(),

            PreviewVar(input) => handler::preview::var::main(&input.selection, &input.query, &input.variable),

            Widget(input) => handler::shell::main(&input.shell).context("Failed to print shell widget code"),

            Fn(input) => handler::func::main(&input.func, input.args.to_vec())
                .with_context(|| format!("Failed to execute function `{:#?}`", input.func)),

            Info(input) => handler::info::main(&input.info)
                .with_context(|| format!("Failed to fetch info `{:#?}`", input.info)),

            #[cfg(not(feature = "disable-repo-management"))]
            Repo(input) => match input.cmd {
                RepoCommand::Add { uri } => {
                    handler::repo::add::main(uri.clone())
                        .with_context(|| format!("Failed to import cheatsheets from `{}`", uri))?;
                    handler::core::main()
                }
                RepoCommand::Browse => {
                    let repo =
                        handler::repo::browse::main().context("Failed to browse featured cheatsheets")?;
                    handler::repo::add::main(repo.clone())
                        .with_context(|| format!("Failed to import cheatsheets from `{}`", repo))?;
                    handler::core::main()
                }
            },
        },
    }
}
