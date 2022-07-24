use crate::prelude::*;
use crate::structures::cheat::VariableMap;
use std::io::Write;

pub trait Fetcher {
    fn fetch(&self, writer: &mut dyn Write, files: &mut Vec<String>) -> Result<Option<VariableMap>>;
}
