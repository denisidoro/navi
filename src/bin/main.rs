extern crate navi;

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
        log::error!("{e:?}");
        FileAnIssue::new(e).into()
    })
}

fn init_logger() -> anyhow::Result<()> {
    let file = std::fs::File::create("navi.log")?;
    env_logger::builder()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();

    Ok(())
}
