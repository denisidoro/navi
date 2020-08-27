use crate::display::Writer;

use crate::structures::cheat::VariableMap;
use crate::structures::config::Config;

use anyhow::Error;

pub mod filesystem;

pub trait Fetcher {
    fn fetch(
        self: &Self,
        config: &Config,
        stdin: &mut std::process::ChildStdin,
        writer: &mut dyn Writer,
    ) -> Result<Option<VariableMap>, Error>;
}
