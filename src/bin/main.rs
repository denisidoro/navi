extern crate navi;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    navi::handle_config(navi::config_from_env())
}
