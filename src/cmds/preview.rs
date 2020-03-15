use std::error::Error;
use std::process;
use crate::display;

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split(display::DELIMITER).skip(3);
    let tags = parts.next().unwrap();
    let comment = parts.next().unwrap();
    let snippet = parts.next().unwrap();
    (tags, comment, snippet)
}

pub fn main(line: String) -> Result<(), Box<dyn Error>> {
    let (tags, comment, snippet) = extract_elements(&line[..]);
    display::preview(comment, tags, snippet);
    process::exit(0)
}
