use std::{fmt::Debug, path::PathBuf};
use thiserror::Error;
#[derive(Error, Debug)]
#[error("Invalid path `{0}`")]
pub struct InvalidPath(pub PathBuf);
