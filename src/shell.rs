use clap::ArgMatches;
use std::error::Error;

pub fn main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let file = match matches.subcommand().1.unwrap().value_of("shell").unwrap() {
        "zsh" => "navi.plugin.zsh",
        "fish" => "navi.plugin.fish",
        _ => "navi.plugin.bash",
    };

    println!(
        "{}/{}",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
        file
    );
    
    Ok(())
}