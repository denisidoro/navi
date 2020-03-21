extern crate navi;

use anyhow::Error;

fn main() -> Result<(), Error> {
    navi::handle_config(navi::config_from_env())
}
