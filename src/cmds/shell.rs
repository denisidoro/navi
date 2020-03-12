use std::error::Error;

use crate::filesystem;

pub fn main(shell: &str) -> Result<(), Box<dyn Error>> {
    let file = match shell {
        "zsh" => "navi.plugin.zsh",
        "fish" => "navi.plugin.fish",
        _ => "navi.plugin.bash",
    };

    println!("{}/{}", filesystem::SHELL_PLUGIN_DIR, file);

    Ok(())
}
