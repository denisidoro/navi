use crate::filesystem::default_cheat_pathbuf;
use crate::fs::pathbuf_to_string;
use anyhow::Error;

#[derive(Debug)]
pub enum Info {
    CheatsPath,
}

pub fn main(info: &Info) -> Result<(), Error> {
    match info {
        Info::CheatsPath => println!("{}", pathbuf_to_string(&default_cheat_pathbuf()?)?),
    }
    Ok(())
}
