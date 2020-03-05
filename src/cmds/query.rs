use std::error::Error;

use crate::cmds;
use crate::cmds::core::Variant;
use crate::option::Config;

pub fn main(query: String, config: Config) -> Result<(), Box<dyn Error>> {
    cmds::core::main(Variant::Query(query), config, true)
}
