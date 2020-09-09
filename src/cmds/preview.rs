use crate::display;
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

pub fn main2(selection: &str, query: &str, variable: &str) -> Result<(), Error> {
    let snippet = env::var("NAVI_PREVIEW_INITIAL_SNIPPET").expect("NAVI_PREVIEW_INITIAL_SNIPPET not set");
    let tags = env::var("NAVI_PREVIEW_TAGS").expect("NAVI_PREVIEW_TAGS not set");
    let comment = env::var("NAVI_PREVIEW_COMMENT").expect("NAVI_PREVIEW_COMMENT not set");
    display::terminal::preview2(&snippet, &tags, &comment, selection, query, variable);
    process::exit(0)
}
