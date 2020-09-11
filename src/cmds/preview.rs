use crate::display;
use crate::env_vars;
use anyhow::Error;
use std::env;
use std::process;

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split(display::DELIMITER).skip(3);
    let tags = parts.next().expect("No `tags` element provided.");
    let comment = parts.next().expect("No `comment` element provided.");
    let snippet = parts.next().expect("No `snippet` element provided.");
    (tags, comment, snippet)
}

pub fn main(line: &str) -> Result<(), Error> {
    let (tags, comment, snippet) = extract_elements(line);
    display::terminal::preview(comment, tags, snippet);
    process::exit(0)
}


pub fn main_var(selection: &str, query: &str, variable: &str) -> Result<(), Error> {
    display::terminal::preview_var(
        selection,
        query,
        variable,
    );
    process::exit(0)
}
