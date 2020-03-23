use crate::flows;
use crate::flows::core::Variant;
use crate::structures::option::Config;
use anyhow::Error;

pub fn main(query: String, config: Config) -> Result<(), Error> {
    flows::core::main(Variant::Query(query), config, true)
}
