use crate::actor;
use crate::config::CONFIG;
use crate::extractor;
use crate::finder::structures::Opts as FinderOpts;
use crate::finder::Finder;
use crate::structures::cheat::VariableMap;
use crate::structures::item::Item;
use crate::writer;
use anyhow::Context;
use anyhow::Result;
use std::io::Write;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts {
        overrides: config.fzf_overrides(),
        select1: false,
        ..Default::default()
    };

    let (raw_selection, variables, files) = config
        .finder()
        .call(opts, |stdin, _| {
            populate_cheatsheet(stdin);
            Ok(Some(VariableMap::new()))
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match());

    if extractions.is_err() {
        return main();
    }

    actor::act(extractions, files, variables)?;
    Ok(())
}

fn add_msg(tags: &str, comment: &str, snippet: &str, stdin: &mut std::process::ChildStdin) {
    let item = Item {
        tags: tags.to_string(),
        comment: comment.to_string(),
        snippet: snippet.to_string(),
        file_index: 0,
    };
    stdin
        .write_all(writer::write(&item).as_bytes())
        .expect("Could not write to fzf's stdin");
}

pub fn populate_cheatsheet(stdin: &mut std::process::ChildStdin) {
    add_msg(
        "cheatsheets",
        "Download default cheatsheets",
        "navi repo add denisidoro/cheats",
        stdin,
    );
    add_msg(
        "cheatsheets",
        "Browse for cheatsheet repos",
        "navi repo browse",
        stdin,
    );
    add_msg("more info", "Read --help message", "navi --help", stdin);
}
