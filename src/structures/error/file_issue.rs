use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(
    "\rHey, listen! Navi encountered a problem.
Do you think this is a bug? File an issue at https://github.com/denisidoro/navi."
)]
pub struct FileAnIssue {
    #[source]
    source: anyhow::Error,
}

impl FileAnIssue {
    pub fn new<SourceError>(source: SourceError) -> Self
    where
        SourceError: Into<anyhow::Error>,
    {
        FileAnIssue {
            source: source.into(),
        }
    }
}
