pub use crate::common::filesystem::{create_dir, exe_string, pathbuf_to_string, remove_dir, InvalidPath, UnreadableDir};
use crate::display::Writer;
pub use crate::fetcher::filesystem::{cheat_pathbuf, read_all};
use crate::fetcher::Fetcher;
use crate::structures::cheat::VariableMap;
use anyhow::Error;

pub fn tmp_path_str() -> Result<String, Error> {
    let cheat_path_str = pathbuf_to_string(cheat_pathbuf()?)?;
    Ok(format!("{}/tmp", cheat_path_str))
}

pub struct Foo {
    path: Option<String>,
}

impl Foo {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl Fetcher for Foo {
    fn fetch(&self, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error> {
        read_all(self.path.clone(), stdin, writer)
    }
}
