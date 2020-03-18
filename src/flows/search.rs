use super::aux;
use crate::structures::option::Config;
use std::error::Error;

pub fn main(_query: String, _config: Config) -> Result<(), Box<dyn Error>> {
    aux::abort("searching for cheats online", 201)
}
