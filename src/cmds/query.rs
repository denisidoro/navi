use crate::cmds;
use crate::cmds::core::Variant;
use crate::structures::config::Config;
use anyhow::Error;

pub fn main(query: String, config: Config) -> Result<(), Error> {
    cmds::core::main(Variant::Query(query), config, true)
}
