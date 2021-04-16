use crate::ui;
use crate::writer;
use anyhow::Result;
use std::process;

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split(writer::DELIMITER).skip(3);
    let tags = parts.next().expect("No `tags` element provided.");
    let comment = parts.next().expect("No `comment` element provided.");
    let snippet = parts.next().expect("No `snippet` element provided.");
    (tags, comment, snippet)
}

pub fn main(line: &str) -> Result<()> {
    let (tags, comment, snippet) = extract_elements(line);

    println!(
        "{comment} {tags} \n{snippet}",
        comment = ui::style(comment).with(*ui::COMMENT_COLOR),
        tags = ui::style(format!("[{}]", tags)).with(*ui::TAG_COLOR),
        snippet = ui::style(writer::fix_newlines(snippet)).with(*ui::SNIPPET_COLOR),
    );

    process::exit(0)
}
