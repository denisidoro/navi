use crate::config::CONFIG;

use crate::finder::Finder;
use crate::handler::core;
use crate::shell::{self, ShellSpawnError};
use crate::structures::cheat::VariableMap;
use crate::url;
use crate::welcome;
use anyhow::Context;
use anyhow::Result;
use std::io::{self, Read};

pub fn map_expand() -> Result<()> {
    let cmd = r#"sed -e 's/^.*$/"&"/' | tr '\n' ' '"#;
    shell::command()
        .arg("-c")
        .arg(cmd)
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;
    Ok(())
}
