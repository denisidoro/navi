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

fn main() -> anyhow::Result<()> {
    if let Err(err) = init_logger() {
        // may need redir stderr to a file to show this log initialization error
        eprintln!("failed to initialize logging: {err:?}");
    }
    navi::handle().map_err(|e| {
        error!("{e:?}");
        FileAnIssue::new(e).into()
    })
}

fn init_logger() -> anyhow::Result<()> {
    const FILE_NAME: &str = "navi.log";
    let mut file = navi::default_config_pathbuf()?;
    file.set_file_name(FILE_NAME);

    // If config path doesn't exist, navi won't log.
    if file.parent().map(|p| !p.exists()).unwrap_or(true) {
        return Ok(());
    }

    let writer = std::fs::File::create(&file).with_context(|| format!("{file:?} is not created"))?;
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(writer)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )?;
    debug!("tracing initialized");

    Ok(())
}
