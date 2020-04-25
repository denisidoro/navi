use super::aux;
use crate::structures::config::Config;
use anyhow::Error;

pub fn main(_query: String, _config: Config) -> Result<(), Error> {
    aux::abort("searching for cheats online", 201)
}
