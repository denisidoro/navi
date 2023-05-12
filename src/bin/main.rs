extern crate navi;

use dns_common::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(
    "\rHey, listen! navi encountered a problem.
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

fn main() -> Result<(), anyhow::Error> {
    init_logger()?;
    navi::handle().map_err(|e| {
        error!("{e:?}");
        FileAnIssue::new(e).into()
    })
}

fn init_logger() -> anyhow::Result<()> {
    const FILE_NAME: &str = "navi.log";
    let mut file = navi::default_config_pathbuf()?;
    file.set_file_name(FILE_NAME);

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(std::fs::File::create(file)?)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )?;
    debug!("tracing initialized");

    Ok(())
}
