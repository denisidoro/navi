use std::error::Error;

use crate::option::Config;

pub fn main(config: Config) -> Result<(), Box<dyn Error>> {
    /*let file = match matches.subcommand().1.unwrap().value_of("shell").unwrap() {
        "zsh" => "navi.plugin.zsh",
        "fish" => "navi.plugin.fish",
        _ => "navi.plugin.bash",
    };

    println!("{}/{}", filesystem::exe_path_string(), file);*/

    Ok(())
}
