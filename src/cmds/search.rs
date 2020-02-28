use std::error::Error;

use crate::option::Config;
use super::aux;

pub fn main(_config: Config) -> Result<(), Box<dyn Error>> {
    aux::abort()
}
