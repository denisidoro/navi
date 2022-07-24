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
use std::process::{Command, Stdio};

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let mut files = vec![];
    let fetcher: Box<dyn Fetcher> = match config.source() {
        Source::Cheats(query) => Box::new(cheatsh::Fetcher::new(query)),
        Source::Tldr(query) => Box::new(tldr::Fetcher::new(query)),
        Source::Filesystem(path, rules) => Box::new(filesystem::Fetcher::new(path, rules)),
    };

    let mut command = Command::new("cat");
    let mut child = command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| anyhow!("Unable to acquire stdin of finder"))?;

    let res = fetcher
        .fetch(stdin, &mut files)
        .context("Failed to parse variables intended for finder")?;

    /*
    if let Some(variables) = res {
        Ok(Some(variables))
    } else {
        welcome::populate_cheatsheet(stdin)?;
        Ok(Some(VariableMap::new()))
    }
    */

    Ok(())
}
