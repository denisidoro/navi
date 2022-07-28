use crate::parser::Parser;
use crate::prelude::*;
use crate::structures::fetcher;

pub fn populate_cheatsheet(parser: &mut Parser) -> Result<()> {
    let cheatsheet = include_str!("../docs/navi.cheat");

    parser.read_lines(
        cheatsheet.split('\n').into_iter().map(|s| Ok(s.to_string())),
        "welcome",
        None,
    )?;

    Ok(())
}

pub struct Fetcher {}

impl Fetcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl fetcher::Fetcher for Fetcher {
    fn fetch(&self, parser: &mut Parser) -> Result<bool> {
        populate_cheatsheet(parser)?;
        Ok(true)
    }
}
