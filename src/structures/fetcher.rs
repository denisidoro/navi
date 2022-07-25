use crate::parser::Parser;
use crate::prelude::*;

pub trait Fetcher {
    fn fetch(&self, parser: &mut Parser, files: &mut Vec<String>) -> Result<bool>;
}
