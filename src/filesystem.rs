use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn exe_string() -> String {
    String::from(
        std::env::current_exe()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
    )
}

pub fn exe_path_string() -> String {
    String::from(
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap(),
    )
}
