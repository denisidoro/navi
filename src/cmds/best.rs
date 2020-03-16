use crate::cmds;
use crate::cmds::core::Variant;
use crate::option::Config;
use std::error::Error;

pub fn main(query: String, args: Vec<String>, config: Config) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        cmds::core::main(Variant::Filter(query), config, false)
    } else {
        cmds::aux::abort("passing arguments to 'navi best'", 201)
    }
}
