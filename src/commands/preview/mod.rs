use crate::deser;
use crate::prelude::*;
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

fn extract_elements(argstr: &str, skip_columns: usize) -> Result<(&str, &str, &str)> {
    let mut parts = argstr.split(deser::terminal::DELIMITER).skip(skip_columns);
    let tags = parts.next().context("No `tags` element provided.")?;
    let comment = parts.next().context("No `comment` element provided.")?;
    let snippet = parts.next().context("No `snippet` element provided.")?;
    Ok((tags, comment, snippet))
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let line = &self.line;

        // In multiline mode, only 1 column is used for rendering tags, comment an snippet with zfs.
        // In legacy mode, using 3 columns for tags, comment and snippet allow permuting their order with --with-nth of fzf.
        let skip_columns = if CONFIG.multiline() { 1 } else { 3 };
        let (tags, comment, snippet) = extract_elements(line, skip_columns)?;

        println!(
            "{comment} {tags} \n{snippet}",
            comment = style(comment).with(CONFIG.comment_color()),
            tags = style(format!("[{tags}]")).with(CONFIG.tag_color()),
            snippet = style(deser::fix_newlines(snippet)).with(CONFIG.snippet_color()),
        );

        process::exit(0)
    }
}
