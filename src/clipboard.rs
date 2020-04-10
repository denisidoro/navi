use crate::structures::error::command::BashSpawnError;
use anyhow::Error;
use std::process::Command;

pub fn copy(text: String) -> Result<(), Error> {
    let cmd = r#"
exst() {
   type "$1" &>/dev/null
}

_paste() {
   if exst pbcopy; then
      pbcopy
   elif exst xclip; then
      xclip -selection clipboard
   elif exst clip.exe; then
      clip.exe
   else
      exit 55
   fi
}
"#;

    Command::new("bash")
        .arg("-c")
        .arg(format!(r#"{} echo "{}" | _paste"#, cmd, text).as_str())
        .spawn()
        .map_err(|e| BashSpawnError::new(cmd, e))?;

    Ok(())
}
