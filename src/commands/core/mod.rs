mod actor;

use crate::clients::{cheatsh, tldr};
use crate::config::Source;
use crate::deser;
use crate::filesystem;
use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::prelude::*;
use crate::structures::fetcher::{Fetcher, StaticFetcher};
use crate::welcome;

pub fn init(fetcher: Box<dyn Fetcher>) -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();
    debug!("opts = {opts:#?}");
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

    debug!(raw_selection = ?raw_selection);
    let extractions = deser::terminal::read(&raw_selection, config.best_match());

    if extractions.is_err() {
        return init(fetcher);
    }

    actor::act(extractions, files, variables)?;

    Ok(())
}

pub fn get_fetcher() -> Result<Box<dyn Fetcher>> {
    let source = CONFIG.source();
    debug!(source = ?source);
    match source {
        Source::Cheats(query) => {
            let lines = cheatsh::call(&query)?;
            let fetcher = Box::new(StaticFetcher::new(lines));
            Ok(fetcher)
        }
        Source::Tldr(query) => {
            let lines = tldr::call(&query)?;
            let fetcher = Box::new(StaticFetcher::new(lines));
            Ok(fetcher)
        }
        Source::Filesystem(path) => {
            let fetcher = Box::new(filesystem::Fetcher::new(path));
            Ok(fetcher)
        }
        Source::Welcome => {
            let fetcher = Box::new(welcome::Fetcher::new());
            Ok(fetcher)
        }
    }
}

pub fn main() -> Result<()> {
    let fetcher = get_fetcher()?;
    init(fetcher)
}
