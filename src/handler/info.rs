use crate::filesystem;
use crate::prelude::*;

#[derive(Debug)]
pub enum Info {
    CheatsExample,
    CheatsPath,
    ConfigPath,
    ConfigExample,
}

pub fn main(info: &Info) -> Result<()> {
    match info {
        Info::CheatsExample => println!("{}", include_str!("../../docs/cheat_example.cheat")),
        Info::CheatsPath => println!("{}", pathbuf_to_string(&filesystem::default_cheat_pathbuf()?)?),
        Info::ConfigPath => println!("{}", pathbuf_to_string(&filesystem::default_config_pathbuf()?)?),
        Info::ConfigExample => println!("{}", include_str!("../../docs/config_file_example.yaml")),
    }
    Ok(())
}
