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

fn get_env_var(name: &str) -> String {
    if let Ok(v) = env::var(name) {
        v
    } else {
        panic!(format!("{} not set", name))
    }
}

pub fn main_var(selection: &str, query: &str, variable: &str) -> Result<(), Error> {
    let snippet = get_env_var(env_vars::PREVIEW_INITIAL_SNIPPET);
    let tags = get_env_var(env_vars::PREVIEW_TAGS);
    let comment = get_env_var(env_vars::PREVIEW_COMMENT);
    let column = display::terminal::parse_env_var(env_vars::PREVIEW_COLUMN);
    let delimiter = env::var(env_vars::PREVIEW_DELIMITER).ok();
    let map = env::var(env_vars::PREVIEW_MAP).ok();
    display::terminal::preview_var(&snippet, &tags, &comment, selection, query, variable, column, delimiter.as_deref(), map.as_deref());
    process::exit(0)
}
