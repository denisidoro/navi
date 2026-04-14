use std::fmt;
use std::fmt::Display;

use clap::Args;

use crate::common::shell::Shell;
use crate::prelude::*;

impl Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Elvish => "elvish",
            Self::Nushell => "nushell",
            Self::Powershell => "powershell",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(ignore_case = true, default_value_t = Shell::Bash)]
    pub shell: Shell,
    /// Custom keybinding for the widget, in shell-native syntax.
    /// Defaults: bash='\C-g', zsh='^g', fish='\cg', elvish='Alt-h', powershell='Ctrl+g'.
    /// For nushell, use 'modifier:keycode' format (e.g., 'control:char_g').
    #[clap(long)]
    pub key: Option<String>,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let shell = &self.shell;

        let content = match shell {
            Shell::Bash => include_str!("../../shell/navi.plugin.bash"),
            Shell::Zsh => include_str!("../../shell/navi.plugin.zsh"),
            Shell::Fish => include_str!("../../shell/navi.plugin.fish"),
            Shell::Elvish => include_str!("../../shell/navi.plugin.elv"),
            Shell::Nushell => include_str!("../../shell/navi.plugin.nu"),
            Shell::Powershell => include_str!("../../shell/navi.plugin.ps1"),
        };

        let content = match shell {
            Shell::Bash => content.replace("__NAVI_KEY__", self.key.as_deref().unwrap_or("\\C-g")),
            Shell::Zsh => content.replace("__NAVI_KEY__", self.key.as_deref().unwrap_or("^g")),
            Shell::Fish => content.replace("__NAVI_KEY__", self.key.as_deref().unwrap_or("\\cg")),
            Shell::Elvish => content.replace("__NAVI_KEY__", self.key.as_deref().unwrap_or("Alt-h")),
            Shell::Nushell => {
                let (modifier, keycode) = if let Some(ref key) = self.key {
                    let parts: Vec<&str> = key.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        (parts[0].trim(), parts[1].trim())
                    } else {
                        ("control", parts[0].trim())
                    }
                } else {
                    ("control", "char_g")
                };
                content
                    .replace("__NAVI_KEY_MODIFIER__", modifier)
                    .replace("__NAVI_KEY_CODE__", keycode)
            }
            Shell::Powershell => content.replace("__NAVI_KEY__", self.key.as_deref().unwrap_or("Ctrl+g")),
        };

        println!("{content}");

        Ok(())
    }
}
