use crate::handler;
use crate::option;
use std::error::Error;
use std::process::Command;

pub fn main(func: String, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match func.as_str() {
        "url::open" => {
            let url = args.into_iter().next().unwrap();
            let cmd = format!("url=\"{}\"; (xdg-open \"$url\" 2> /dev/null || open \"$url\" 2> /dev/null) &disown", url);
            Command::new("bash")
                .arg("-c")
                .arg(cmd.as_str())
                .spawn()
                .unwrap();
            Ok(())
        }

        "welcome" => handler::handle_config(option::config_from_iter(
            "navi --path /tmp/irrelevant".split(' ').collect(),
        )),

        _ => unreachable!(""),
    }
}
