use clap::ArgMatches;
use std::error::Error;
use std::process;

pub fn main(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    eprintln!("This version of navi doesn't support this command.");
    eprintln!("Please check https://github.com/denisidoro/navi/releases/tag/v1.0.0 for more info.");
    process::exit(42)
}
