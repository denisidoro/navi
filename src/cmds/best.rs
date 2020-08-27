use crate::cmds;
use crate::cmds::core::Variant;
use crate::structures::config::Config;
use anyhow::Error;

pub fn main(query: String, args: Vec<String>, config: Config) -> Result<(), Error> {
    if args.is_empty() {
        cmds::core::main(Variant::Filter(query), config, false)
    } else {
        cmds::aux::abort("passing arguments to 'navi best'", 201)
    }
}
