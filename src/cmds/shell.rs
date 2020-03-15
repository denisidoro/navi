use std::error::Error;

pub fn main(shell: &str) -> Result<(), Box<dyn Error>> {
    let content = match shell {
        "zsh" => include_str!("../../shell/navi.plugin.zsh"),
        "fish" => include_str!("../../shell/navi.plugin.fish"),
        _ => include_str!("../../shell/navi.plugin.bash"),
    };

    println!("{}", content);

    Ok(())
}
