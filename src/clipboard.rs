use crate::shell::{self, ShellSpawnError, EOF};
use anyhow::Result;

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

    shell::command()
        .arg("-c")
        .arg(
            format!(
                r#"{cmd} 
        read -r -d '' x <<'{eof}'
{text}
{eof}

echo -n "$x" | _copy"#,
                cmd = cmd,
                text = text,
                eof = EOF,
            )
            .as_str(),
        )
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;

    Ok(())
}
