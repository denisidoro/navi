use crate::prelude::*;
use crate::shell::Shell;

pub fn main(shell: &Shell) -> Result<()> {
    let content = match shell {
        Shell::Bash => include_str!("../../shell/navi.plugin.bash"),
        Shell::Zsh => include_str!("../../shell/navi.plugin.zsh"),
        Shell::Fish => include_str!("../../shell/navi.plugin.fish"),
        Shell::Elvish => include_str!("../../shell/navi.plugin.elv"),
    };

    println!("{}", content);

    Ok(())
}
