use crate::structures::cheat::VariableMap;
use crate::writer::Writer;
use anyhow::Result;

pub trait Fetcher {
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        writer: &mut dyn Writer,
        files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>>;
}
