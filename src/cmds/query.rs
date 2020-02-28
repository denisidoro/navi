use std::error::Error;

use super::aux;
use crate::option::Config;

pub fn main(_config: Config) -> Result<(), Box<dyn Error>> {
    aux::abort()
}
