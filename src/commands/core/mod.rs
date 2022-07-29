mod actor;
mod extractor;

use crate::filesystem;
use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::prelude::*;
use crate::structures::fetcher::Fetcher;
use crate::welcome;

pub fn init(fetcher: Box<dyn Fetcher>) -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();
    // let fetcher = config.fetcher();

    let (raw_selection, (variables, files)) = config
        .finder()
        .call(opts, |writer| {
            let mut parser = Parser::new(writer, true);

            let found_something = fetcher
                .fetch(&mut parser)
                .context("Failed to parse variables intended for finder")?;

            if !found_something {
                welcome::populate_cheatsheet(&mut parser)?;
            }

            Ok((Some(parser.variables), fetcher.files()))
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match());

    if extractions.is_err() {
        return main(fetcher);
    }

    actor::act(extractions, files, variables)?;

    Ok(())
}

pub fn main(fetcher: Box<dyn Fetcher>) -> Result<()> {
    let config = &CONFIG;
    let fetcher = Box::new(filesystem::Fetcher::new(config.path()));
    init(fetcher)
}
