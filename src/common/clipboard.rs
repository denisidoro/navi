use crate::common::shell::{self, ShellSpawnError, EOF};
use crate::prelude::*;

pub fn copy(text: String) -> Result<()> {
    let cmd = r#"
exst() {
   type "$1" &>/dev/null
}

_copy() {
   if exst pbcopy; then
      pbcopy
   elif exst xclip; then
      xclip -selection clipboard
   elif exst clip.exe; then
      clip.exe
   else
      exit 55
   fi
}"#;

    shell::out()
        .arg(
            format!(
                r#"{cmd} 
        read -r -d '' x <<'{EOF}'
{text}
{EOF}

echo -n "$x" | _copy"#,
            )
            .as_str(),
        )
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;

    Ok(())
}
