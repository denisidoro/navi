use crate::flows;
use crate::flows::core::Variant;
use crate::option::Config;
use std::error::Error;

pub fn main(query: String, config: Config) -> Result<(), Box<dyn Error>> {
    flows::core::main(Variant::Query(query), config, true)
}
