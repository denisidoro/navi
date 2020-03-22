use std::{fmt::Debug, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid path `{0}`")]
pub struct InvalidPath(pub PathBuf);

#[derive(Error, Debug)]
#[error("Unable to read directory `{dir}`")]
pub struct UnreadableDir {
    dir: PathBuf,
    #[source]
    source: anyhow::Error,
}

impl UnreadableDir {
    pub fn new<DirT, SourceError>(dir: DirT, source: SourceError) -> Self
    where
        DirT: Into<PathBuf>,
        SourceError: std::error::Error + Sync + Send + 'static,
    {
        UnreadableDir {
            dir: dir.into(),
            source: source.into(),
        }
    }
}
