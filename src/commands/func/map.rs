use crate::common::shell::{self, ShellSpawnError};
use crate::prelude::*;

pub fn expand() -> Result<()> {
    let cmd = r#"sed -e 's/^.*$/"&"/' | tr '\n' ' '"#;
    shell::out()
        .arg(cmd)
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;
    Ok(())
}
