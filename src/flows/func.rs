use crate::handler;
use crate::structures::{error::command::BashSpawnError, option};
use anyhow::Error;
use std::process::Command;

pub fn main(func: String, args: Vec<String>) -> Result<(), Error> {
    match func.as_str() {
        "url::open" => {
            let url = args
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("No URL specified"))?;
            let cmd = format!("url=\"{}\"; (xdg-open \"$url\" 2> /dev/null || open \"$url\" 2> /dev/null) &disown", url);
            Command::new("bash")
                .arg("-c")
                .arg(cmd.as_str())
                .spawn()
                .map_err(|e| BashSpawnError::new(cmd, e))?;
            Ok(())
        }

        "welcome" => handler::handle_config(option::config_from_iter(
            "navi --path /tmp/irrelevant".split(' ').collect(),
        )),

        _ => Err(anyhow!("Unrecognized function")),
    }
}
