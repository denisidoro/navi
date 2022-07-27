use crate::clients::cheatsh;
use crate::clients::tldr;

use crate::config::Source;
use crate::filesystem;
use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::prelude::*;
use crate::structures::fetcher::Fetcher;
use std::io::{stdout, Write};

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let _opts = FinderOpts::snippet_default();

    let mut fetcher: Box<dyn Fetcher> = match config.source() {
        Source::Cheats(query) => Box::new(cheatsh::Fetcher::new(query)),
        Source::Tldr(query) => Box::new(tldr::Fetcher::new(query)),
        Source::Filesystem(path, rules) => Box::new(filesystem::Fetcher::new(path, rules)),
    };

    let hash: u64 = 1531163706200719240;

    let mut stdout = stdout();
    let mut writer: Box<&mut dyn Write> = Box::new(&mut stdout);
    let mut parser = Parser::new(&mut writer, false);

    let _res = fetcher
        .fetch(&mut parser)
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
