use crate::handler;
use crate::structures::config;
use crate::common::url;
use anyhow::Error;

pub fn main(func: String, args: Vec<String>) -> Result<(), Error> {
    match func.as_str() {
        "url::open" => url::open(args),

        "welcome" => handler::handle_config(config::config_from_iter(
            "navi --path /tmp/navi/irrelevant".split(' ').collect(),
        )),

        _ => Err(anyhow!("Unrecognized function")),
    }
}
