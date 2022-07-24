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
        Info::CheatsPath => println!("{}", &filesystem::default_cheat_pathbuf()?.to_string()),
        Info::ConfigPath => println!("{}", &filesystem::default_config_pathbuf()?.to_string()),
        Info::ConfigExample => println!("{}", include_str!("../../docs/config_file_example.yaml")),
    }
    Ok(())
}
