use std::{fmt::Debug, path::PathBuf};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum FilesystemError {
    #[error("Invalid path `{0}`")]
    InvalidPath(PathBuf),
}
