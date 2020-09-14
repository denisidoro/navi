pub use crate::common::filesystem::{create_dir, exe_string, pathbuf_to_string, remove_dir, InvalidPath, UnreadableDir};
use crate::display::Writer;
use crate::fetcher;
pub use crate::fetcher::filesystem::{default_cheat_pathbuf, read_all, all_cheat_files};
use crate::structures::cheat::VariableMap;
use anyhow::Error;

pub fn tmp_path_str() -> Result<String, Error> {
    let cheat_path_str = pathbuf_to_string(default_cheat_pathbuf()?)?;
    Ok(format!("{}/tmp", cheat_path_str))
}

pub struct Fetcher {
    path: Option<String>,
}

impl Fetcher {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl fetcher::Fetcher for Fetcher {
    fn fetch(&self, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error> {
        read_all(self.path.clone(), stdin, writer)
    }
}
