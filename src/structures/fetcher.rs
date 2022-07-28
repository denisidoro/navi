use crate::parser::Parser;
use crate::prelude::*;

pub trait Fetcher {
    fn fetch(&self, parser: &mut Parser) -> Result<bool>;

    fn files(&self) -> Vec<String> {
        vec![]
    }
}
