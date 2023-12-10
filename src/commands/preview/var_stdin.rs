use clap::Args;

use super::var;
use crate::common::shell::{self, ShellSpawnError, EOF};
use crate::prelude::*;
use std::io::{self, Read};

#[derive(Debug, Clone, Args)]
pub struct Input {}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let mut text = String::new();
        io::stdin().read_to_string(&mut text)?;

        let mut parts = text.split(EOF);
        let selection = parts.next().expect("Unable to get selection").to_owned();
        let query = parts.next().expect("Unable to get query").to_owned();
        let variable = parts.next().expect("Unable to get variable").trim().to_owned();

        let input = var::Input {
            selection,
            query,
            variable,
        };

        input.run()?;

        if let Some(extra) = parts.next() {
            if !extra.is_empty() {
                print!("");

                let mut cmd = shell::out();
                cmd.arg(extra);
                debug!(?cmd);
                cmd.spawn().map_err(|e| ShellSpawnError::new(extra, e))?.wait()?;
            }
        }

        Ok(())
    }
}
