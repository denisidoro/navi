pub mod filesystem;

use crate::display::Writer;
use crate::structures::cheat::VariableMap;
use anyhow::Error;

pub trait Fetcher {
    fn fetch(&self, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error>;
}
