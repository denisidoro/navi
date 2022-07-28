mod actor;
mod extractor;

use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::prelude::*;
use crate::welcome;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let (raw_selection, (variables, files)) = config
        .finder()
        .call(opts, |writer| {
            let fetcher = config.fetcher();

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
        return main();
    }

    actor::act(extractions, files, variables)?;

    Ok(())
}