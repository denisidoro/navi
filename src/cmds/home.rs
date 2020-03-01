use std::error::Error;

use crate::filesystem;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", filesystem::exe_path_string());
    Ok(())
}
