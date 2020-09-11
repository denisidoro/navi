use crate::display;
use anyhow::Error;
use std::env;
use std::process;
use crate::env_vars;

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

fn get_env_var(name: &str) -> String {
    if let Ok(v) = env::var(name) {
        v
    } else {
            panic!(format!("{} not set", name))
        }
}

pub fn main2(selection: &str, query: &str, variable: &str) -> Result<(), Error> {
    let snippet = get_env_var(env_vars::PREVIEW_INITIAL_SNIPPET);
    let tags = get_env_var(env_vars::PREVIEW_TAGS);
    let comment = get_env_var(env_vars::PREVIEW_COMMENT);
    display::terminal::preview2(&snippet, &tags, &comment, selection, query, variable);
    process::exit(0)
}
