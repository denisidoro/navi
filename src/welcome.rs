use crate::config::CONFIG;
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
    let opts = FinderOpts::from_config(&config)?;
    let _ = config
        .finder
        .call(opts, |stdin, _| {
            populate_cheatsheet(stdin);
            Ok(Some(VariableMap::new()))
        })
        .context("Failed getting selection and variables from finder")?;
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
