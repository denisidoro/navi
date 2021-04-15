use crate::env_var;
use std::fmt::Debug;
use std::process::Command;
use thiserror::Error;

lazy_static! {
    pub static ref IS_FISH: bool = env_var::get("SHELL")
        .unwrap_or_else(|_| "".to_string())
        .contains(&"fish");
    static ref SHELL: String = env_var::get(env_var::SHELL).unwrap_or_else(|_| "bash".to_string());
}

#[derive(Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
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

pub fn command() -> Command {
    Command::new(&*SHELL)
}
