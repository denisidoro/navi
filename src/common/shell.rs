use std::env;
use std::fmt::Debug;
use thiserror::Error;

lazy_static! {
    pub static ref IS_FISH: bool = env::var("SHELL").unwrap_or("".to_string()).contains(&"fish");
}

#[derive(Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

#[derive(Error, Debug)]
#[error("Failed to spawn child process `bash` to execute `{command}`")]
pub struct BashSpawnError {
    command: String,
    #[source]
    source: anyhow::Error,
}

impl BashSpawnError {
    pub fn new<SourceError>(command: impl Into<String>, source: SourceError) -> Self
    where
        SourceError: std::error::Error + Sync + Send + 'static,
    {
        BashSpawnError {
            command: command.into(),
            source: source.into(),
        }
    }
}
