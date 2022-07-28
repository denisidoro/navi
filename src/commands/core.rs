use crate::actor;
use crate::extractor;
use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::prelude::*;
use crate::welcome;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let files = vec![];

    let (raw_selection, variables) = config
        .finder()
        .call(opts, |writer| {
            let mut fetcher = config.fetcher();

            let mut parser = Parser::new(writer, true, config.tag_rules());

            let found_something = fetcher
                .fetch(&mut parser)
                .context("Failed to parse variables intended for finder")?;

            if !found_something {
                welcome::populate_cheatsheet(&mut parser)?;
            }

            // files = fetcher.files(); // TODO
            Ok(Some(parser.variables))
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match());

    if extractions.is_err() {
        return main();
    }

    actor::act(extractions, files, variables)?;

    Ok(())
}
