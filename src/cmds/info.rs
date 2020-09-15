use anyhow::Error;

#[derive(Debug)]
pub enum Info {
    CheatsPath,
}

pub fn main(info: &Info) -> Result<(), Error> {
    match info {
        Info::CheatsPath => println!("foo"),
    }
    Ok(())
}
