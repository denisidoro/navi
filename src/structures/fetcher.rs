use crate::structures::cheat::VariableMap;
use anyhow::Result;

pub trait Fetcher {
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>>;
}
