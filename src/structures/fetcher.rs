use crate::prelude::*;
use crate::structures::cheat::VariableMap;

pub trait Fetcher {
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>>;
}
