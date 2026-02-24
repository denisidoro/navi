use crate::common::shell::{self, ShellSpawnError, EOF};
use crate::prelude::*;

pub fn copy(text: String) -> Result<()> {
   let shell_cmd = CONFIG.shell().to_lowercase();
   if shell_cmd.contains("powershell") || shell_cmd.contains("cmd.exe") {
      // Use Windows native clipboard
      let mut cmd = std::process::Command::new("cmd.exe");
      cmd.arg("/C").arg("clip.exe");
      let mut child = cmd.stdin(std::process::Stdio::piped()).spawn()
         .map_err(|e| ShellSpawnError::new("clip.exe", e))?;
      if let Some(stdin) = child.stdin.as_mut() {
         use std::io::Write;
         stdin.write_all(text.as_bytes())?;
      }
      child.wait()?;
      Ok(())
   } else {
      // Use bash/zsh/fish/etc logic
      let script = format!(
         r#"exst() {{ type "$1" &>/dev/null; }}
_copy() {{
  if exst pbcopy; then pbcopy
  elif exst xclip; then xclip -selection clipboard
  elif exst clip.exe; then clip.exe
  else exit 55; fi
}}
read -r -d '' x <<"{EOF}"
{text}
{EOF}
echo -n "$x" | _copy
"#,
         EOF = EOF,
         text = text
      );
      shell::out()
         .arg(script.as_str())
         .spawn()
         .map_err(|e| ShellSpawnError::new(script.as_str(), e))?
         .wait()?;
      Ok(())
   }
}
