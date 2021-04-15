use crate::shell::{self, ShellSpawnError};
use anyhow::Error;

pub fn copy(text: String) -> Result<(), Error> {
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
                r#"{} 
        read -r -d '' x <<'NAVIEOF'
{}
NAVIEOF

echo -n "$x" | _copy"#,
                cmd, text
            )
            .as_str(),
        )
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;

    Ok(())
}
