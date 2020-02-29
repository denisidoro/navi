use std::error::Error;

use crate::cmds;
use crate::option::Config;

pub fn main(args: Vec<String>, config: Config) -> Result<(), Box<dyn Error>> {
    cmds::core::main(&args.join(" "), config)
}
