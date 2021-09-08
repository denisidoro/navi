use crate::config::CONFIG;
use anyhow::Result;
use std::fmt::Debug;
use std::io::{self, Read};
use std::process::Command;
use thiserror::Error;

pub const EOF: &str = "NAVIEOF";

#[derive(Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Elvish,
}

#[derive(Error, Debug)]
#[error("Failed to spawn child process `bash` to execute `{command}`")]
pub struct ShellSpawnError {
    command: String,
    #[source]
    source: anyhow::Error,
}

impl ShellSpawnError {
    pub fn new<SourceError>(command: impl Into<String>, source: SourceError) -> Self
    where
        SourceError: std::error::Error + Sync + Send + 'static,
    {
        ShellSpawnError {
            command: command.into(),
            source: source.into(),
        }
    }
}

pub fn out() -> Command {
    let shell = CONFIG.shell();
    let mut cmd = Command::new(&shell);
    let arg = if shell == "cmd.exe" { "/c" } else { "-c" };
    cmd.arg(arg);
    cmd
}

pub fn widget_last_command() -> Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    let replacements = vec![("||", "ග"), ("|", "ඛ"), ("&&", "ඝ")];

    let parts = shellwords::split(&text).unwrap_or_else(|_| text.split('|').map(|s| s.to_string()).collect());

    for p in parts {
        for (pattern, escaped) in replacements.clone() {
            if p.contains(pattern) && p != pattern && p != format!("{}{}", pattern, pattern) {
                let replacement = p.replace(pattern, escaped);
                text = text.replace(&p, &replacement);
            }
        }
    }

    let mut extracted = text.clone();

    for (pattern, _) in replacements.clone() {
        let mut new_parts = text.rsplit(pattern);
        if let Some(extracted_attempt) = new_parts.next() {
            if extracted_attempt.len() <= extracted.len() {
                extracted = extracted_attempt.to_string();
            }
        }
    }

    for (pattern, escaped) in replacements.clone() {
        text = text.replace(&escaped, pattern);
        extracted = extracted.replace(&escaped, pattern);
    }

    println!("{}", extracted.trim_start());

    Ok(())
}
