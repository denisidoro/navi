use crate::prelude::*;
use crate::shell::{self, ShellSpawnError, EOF};
use std::io::{self, Read};

use super::var::Input;

pub fn main() -> Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    let mut parts = text.split(EOF);
    let selection = parts.next().expect("Unable to get selection").to_owned();
    let query = parts.next().expect("Unable to get query").to_owned();
    let variable = parts.next().expect("Unable to get variable").trim().to_owned();

    let input = Input {
        selection,
        query,
        variable,
    };

    super::var::main(&input)?;

    if let Some(extra) = parts.next() {
        if !extra.is_empty() {
            print!("");

            shell::out()
                .arg(extra)
                .spawn()
                .map_err(|e| ShellSpawnError::new(extra, e))?
                .wait()?;
        }
    }

    Ok(())
}
