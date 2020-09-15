use anyhow::Error;
use crate::common::shell::Shell;

pub fn main(shell: &Shell) -> Result<(), Error> {
    let content = match shell {
        Shell::Bash => include_str!("../../shell/navi.plugin.bash"),
        Shell::Zsh => include_str!("../../shell/navi.plugin.zsh"),
        Shell::Fish => include_str!("../../shell/navi.plugin.fish"),
    };

    println!("{}", content);

    Ok(())
}
