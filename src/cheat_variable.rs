use crate::shell::{self, ShellSpawnError};

use anyhow::Result;

pub fn map_expand() -> Result<()> {
    let cmd = r#"sed -e 's/^.*$/"&"/' | tr '\n' ' '"#;
    shell::out()
        .arg(cmd)
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;
    Ok(())
}
