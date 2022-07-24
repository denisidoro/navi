pub mod core;
pub mod func;
pub mod info;
pub mod preview;
pub mod repo;
pub mod shell;

use crate::handler;
use crate::prelude::*;

pub fn handle() -> Result<()> {
    use crate::config::Command::*;

    match CONFIG.cmd() {
        None => handler::core::main(),

        Some(c) => match c {
            Preview(input) => handler::preview::main(input),

            PreviewVarStdin => handler::preview::var_stdin::main(),

            PreviewVar(input) => handler::preview::var::main(input),

            Widget(input) => handler::shell::main(input).context("Failed to print shell widget code"),

            Fn(input) => handler::func::main(input)
                .with_context(|| format!("Failed to execute function `{:#?}`", input.func)),

            Info(input) => handler::info::main(input)
                .with_context(|| format!("Failed to fetch info `{:#?}`", input.info)),

            #[cfg(not(feature = "disable-repo-management"))]
            Repo(input) => handler::repo::main(input),
        },
    }
}
