use crate::writer;

use anyhow::Context;
use anyhow::Result;

pub type Output<'a> = (&'a str, &'a str, &'a str, &'a str, Option<usize>);

pub fn extract_from_selections(raw_snippet: &str, is_single: bool) -> Result<Output> {
    let mut lines = raw_snippet.split('\n');
    let key = if is_single {
        "enter"
    } else {
        lines
            .next()
            .context("Key was promised but not present in `selections`")?
    };

    let mut parts = lines
        .next()
        .context("No more parts in `selections`")?
        .split(writer::DELIMITER)
        .skip(3);

    let tags = parts.next().unwrap_or("");
    let comment = parts.next().unwrap_or("");
    let snippet = parts.next().unwrap_or("");
    let file_index = parts.next().unwrap_or("").parse().ok();
    Ok((key, tags, comment, snippet, file_index))
}
