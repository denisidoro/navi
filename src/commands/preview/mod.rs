use crate::prelude::*;
use crate::serializer;
use clap::Args;
use crossterm::style::{style, Stylize};
use std::process;

pub mod var;
pub mod var_stdin;

#[derive(Debug, Clone, Args)]
pub struct Input {
    /// Selection line
    pub line: String,
}

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split(serializer::DELIMITER).skip(3);
    let tags = parts.next().expect("No `tags` element provided.");
    let comment = parts.next().expect("No `comment` element provided.");
    let snippet = parts.next().expect("No `snippet` element provided.");
    (tags, comment, snippet)
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let line = &self.line;

        let (tags, comment, snippet) = extract_elements(line);

        println!(
            "{comment} {tags} \n{snippet}",
            comment = style(comment).with(CONFIG.comment_color()),
            tags = style(format!("[{}]", tags)).with(CONFIG.tag_color()),
            snippet = style(serializer::fix_newlines(snippet)).with(CONFIG.snippet_color()),
        );

        process::exit(0)
    }
}
