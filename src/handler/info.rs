use crate::filesystem;
use crate::fs::pathbuf_to_string;
use anyhow::Result;

#[derive(Debug)]
pub enum Info {
    CheatsPath,
    ConfigPath,
}

pub fn main(info: &Info) -> Result<()> {
    match info {
        Info::CheatsPath => println!("{}", pathbuf_to_string(&filesystem::default_cheat_pathbuf()?)?),
        Info::ConfigPath => println!("{}", pathbuf_to_string(&filesystem::default_config_pathbuf()?)?),
    }
    Ok(())
}
