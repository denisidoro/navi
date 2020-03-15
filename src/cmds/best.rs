use std::error::Error;
use crate::cmds;
use crate::cmds::core::Variant;
use crate::option::Config;

pub fn main(query: String, args: Vec<String>, config: Config) -> Result<(), Box<dyn Error>> {
    if !args.is_empty() {
        cmds::aux::abort("passing arguments to 'navi best'", 201)
    } else {
        cmds::core::main(Variant::Filter(query), config, false)
    }
}
