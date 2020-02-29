use std::error::Error;

use crate::cmds;
use crate::cmds::core::Variant;
use crate::option::Config;

pub fn main(args: Vec<String>, config: Config) -> Result<(), Box<dyn Error>> {
    cmds::core::main(Variant::Filter(args.join(" ")), config)
}
