use std::io::Write;

use crate::actor;
use crate::clients::cheatsh;
use crate::clients::tldr;
use crate::config::Source;
use crate::extractor;
use crate::filesystem;
use crate::finder::structures::Opts as FinderOpts;
use crate::finder::Finder;
use crate::prelude::*;
use crate::structures::cheat::VariableMap;
use crate::structures::fetcher::Fetcher;
use crate::welcome;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let (raw_selection, variables, files) = config
        .finder()
        .call(opts, |stdin, files| {
            let fetcher: Box<dyn Fetcher> = match config.source() {
                Source::Cheats(query) => Box::new(cheatsh::Fetcher::new(query)),
                Source::Tldr(query) => Box::new(tldr::Fetcher::new(query)),
                Source::Filesystem(path, rules) => Box::new(filesystem::Fetcher::new(path, rules)),
            };

            let mut writer: Box<&mut dyn Write> = Box::new(stdin);

            let res = fetcher
                .fetch(&mut writer, files)
                .context("Failed to parse variables intended for finder")?;

            if let Some(variables) = res {
                Ok(Some(variables))
            } else {
                welcome::populate_cheatsheet(&mut writer)?;
                Ok(Some(VariableMap::new()))
            }
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match());

    if extractions.is_err() {
        return main();
    }

    actor::act(extractions, files, variables)?;

    Ok(())
}
