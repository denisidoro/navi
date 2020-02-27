use clap::ArgMatches;
use std::error::Error;

use super::aux;

pub fn main(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    aux::abort()
}
