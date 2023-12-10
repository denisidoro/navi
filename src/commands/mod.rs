pub mod core;
pub mod func;
pub mod info;
pub mod preview;
pub mod repo;
pub mod shell;
pub mod temp;

use crate::commands;
use crate::prelude::*;

pub fn handle() -> Result<()> {
    use crate::config::Command::*;

    debug!("CONFIG = {:#?}", &*CONFIG);
    match CONFIG.cmd() {
        None => commands::core::main(),

        Some(c) => match c {
            Preview(input) => input.run(),

            PreviewVarStdin(input) => input.run(),

            PreviewVar(input) => input.run(),

            Widget(input) => input.run().context("Failed to print shell widget code"),

            Fn(input) => input
                .run()
                .with_context(|| format!("Failed to execute function `{:#?}`", input.func)),

            Info(input) => input
                .run()
                .with_context(|| format!("Failed to fetch info `{:#?}`", input.info)),

            #[cfg(not(feature = "disable-repo-management"))]
            Repo(input) => input.run(),
        },
    }
}
