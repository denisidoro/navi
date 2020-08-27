pub use crate::common::filesystem::{
    create_dir, exe_string, pathbuf_to_string, remove_dir, InvalidPath, UnreadableDir,
};
use crate::display::Writer;
use crate::structures::cheat::VariableMap;
use crate::structures::config::Config;
use anyhow::Error;

pub use crate::fetcher::filesystem::{cheat_pathbuf, read_all};
use crate::fetcher::Fetcher;

pub fn tmp_path_str() -> Result<String, Error> {
    let cheat_path_str = pathbuf_to_string(cheat_pathbuf()?)?;
    Ok(format!("{}/tmp", cheat_path_str))
}

pub struct Foo {}

impl Foo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fetcher for Foo {
    fn fetch(
        &self,
        config: &Config,
        stdin: &mut std::process::ChildStdin,
        writer: &mut dyn Writer,
    ) -> Result<VariableMap, Error> {
        read_all(config, stdin, writer)
    }
}
