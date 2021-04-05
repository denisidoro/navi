use crate::actor;
use crate::cheatsh;

use crate::display;

use crate::extractor;
use crate::filesystem;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::Finder;
use crate::structures::fetcher::Fetcher;

use crate::structures::cheat::VariableMap;

use crate::structures::config::Config;
use crate::structures::config::Source;
use crate::tldr;
use crate::welcome;
use anyhow::Context;
use anyhow::Error;

fn gen_core_finder_opts(config: &Config) -> Result<FinderOpts, Error> {
    let opts = FinderOpts {
        preview: if config.no_preview {
            None
        } else {
            Some(format!("{} preview {{}}", filesystem::exe_string()?))
        },
        overrides: config.fzf_overrides.clone(),
        suggestion_type: SuggestionType::SnippetSelection,
        query: if config.best_match {
            None
        } else {
            config.get_query()
        },
        filter: if config.best_match {
            config.get_query()
        } else {
            None
        },
        ..Default::default()
    };

    Ok(opts)
}

pub fn main(config: Config) -> Result<(), Error> {
    let opts = gen_core_finder_opts(&config).context("Failed to generate finder options")?;

    let mut files = Vec::new();

    let (raw_selection, variables) = config
        .finder
        .call(opts, &mut files, |stdin, files| {
            let mut writer = display::terminal::Writer::new();

            let fetcher: Box<dyn Fetcher> = match config.source() {
                Source::Cheats(query) => Box::new(cheatsh::Fetcher::new(query)),
                Source::Tldr(query) => Box::new(tldr::Fetcher::new(query)),
                Source::Filesystem(path) => Box::new(filesystem::Fetcher::new(path)),
            };

            let res = fetcher
                .fetch(stdin, &mut writer, files)
                .context("Failed to parse variables intended for finder")?;

            if let Some(variables) = res {
                Ok(Some(variables))
            } else {
                welcome::populate_cheatsheet(&mut writer, stdin);
                Ok(Some(VariableMap::new()))
            }
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match);

    if extractions.is_err() {
        return main(config);
    }

    actor::act(extractions, config, &mut files, variables)?;

    Ok(())
}
