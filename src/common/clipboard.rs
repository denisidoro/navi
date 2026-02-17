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
      let mut script = String::new();
      script.push_str("exst() { type \"$1\" &>/dev/null; }\n");
      script.push_str("_copy() {\n");
      script.push_str("  if exst pbcopy; then pbcopy\n");
      script.push_str("  elif exst xclip; then xclip -selection clipboard\n");
      script.push_str("  elif exst clip.exe; then clip.exe\n");  // # Could be used on WSL or GitBash
      script.push_str("  else exit 55; fi\n");
      script.push_str("}\n");
      script.push_str(&format!(
         "read -r -d '' x <<\"{EOF}\"\n{text}\n{EOF}\n",
         EOF = EOF,
         text = text
      ));
      script.push_str("echo -n \"$x\" | _copy\n");
      shell::out()
         .arg(script.as_str())
         .spawn()
         .map_err(|e| ShellSpawnError::new(script.as_str(), e))?
         .wait()?;
      Ok(())
   }
}
