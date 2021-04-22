use crate::shell::{self, ShellSpawnError, EOF};
use anyhow::Result;
use std::io::{self, Read};

pub fn main() -> Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    let mut parts = text.split(EOF);
    let selection = parts.next().expect("Unable to get selection");
    let query = parts.next().expect("Unable to get query");
    let variable = parts.next().expect("Unable to get variable").trim();

    super::handler::preview_var::main(selection, query, variable)?;

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
