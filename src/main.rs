use std::error::Error;

mod cmds;
mod filesystem;
mod fzf;
mod option;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let config = option::parse();

    println!("{:#?}", config);

    /* match matches.subcommand().0 {
        "preview" => cmds::preview::main(&matches),
        "widget" => cmds::shell::main(&matches),
        "best" => cmds::best::main(&matches),
        "search" => cmds::search::main(&matches),
        "query" => cmds::query::main(&matches),
        _ => cmds::core::main(&matches),
    }*/

    Ok(())
}
