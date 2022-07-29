use crate::parser::Parser;
use crate::prelude::*;

pub trait Fetcher {
    fn fetch(&self, parser: &mut Parser) -> Result<bool>;

    fn files(&self) -> Vec<String> {
        vec![]
    }
}

pub struct StaticFetcher {
    lines: Box<dyn Iterator<Item = Result<String>>>,
}

impl StaticFetcher {
    pub fn new(lines: Box<dyn Iterator<Item = Result<String>>>) -> Self {
        Self { lines }
    }
}

impl Fetcher for StaticFetcher {
    fn fetch(&self, parser: &mut Parser) -> Result<bool> {
        parser.read_lines(self.lines, "static", None)?;
        Ok(true)
    }
}
