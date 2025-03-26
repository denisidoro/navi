use super::*;
use crate::common::terminal;
use crate::structures::item::Item;
use crossterm::style::{style, Stylize};
use std::cmp::max;

pub fn get_widths() -> (usize, usize, usize) {
    let width = terminal::width();
    let tag_width_percentage = max(
        CONFIG.tag_min_width(),
        width * CONFIG.tag_width_percentage() / 100,
    );
    let comment_width_percentage = max(
        CONFIG.comment_min_width(),
        width * CONFIG.comment_width_percentage() / 100,
    );
    let snippet_width_percentage = max(
        CONFIG.snippet_min_width(),
        width * CONFIG.snippet_width_percentage() / 100,
    );
    (
        usize::from(tag_width_percentage),
        usize::from(comment_width_percentage),
        usize::from(snippet_width_percentage),
    )
}

pub const DELIMITER: &str = r"  ⠀";

lazy_static! {
    pub static ref COLUMN_WIDTHS: (usize, usize, usize) = get_widths();
}

pub fn write(item: &Item) -> String {
    let (tag_width_percentage, comment_width_percentage, snippet_width_percentage) = *COLUMN_WIDTHS;

    let separator_count = max(
        item.snippet.matches(LINE_SEPARATOR).count(),
        item.comment.matches(LINE_SEPARATOR).count(),
    );

    let splitted_comment = item.comment.split(LINE_SEPARATOR).collect::<Vec<&str>>();
    let splitted_snippet = item.snippet.split(LINE_SEPARATOR).collect::<Vec<&str>>();

    let printer_item = (0..=separator_count)
        .map(|i| {
            format!(
                "{tags_short}{delimiter}{comment_line_i}{delimiter}{snippet_line_i}",
                tags_short     = style(limit_str(
                    if i == 0 { &item.tags } else { "" },
                    tag_width_percentage
                ))
                .with(CONFIG.tag_color()),
                comment_line_i = style(limit_str(
                    splitted_comment.get(i).unwrap_or(&""),
                    comment_width_percentage
                ))
                .with(CONFIG.comment_color()),
                snippet_line_i = style(limit_str(
                    splitted_snippet.get(i).unwrap_or(&""),
                    snippet_width_percentage
                ))
                .with(CONFIG.snippet_color()),
                delimiter = " ",
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
            "{printer_item}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}{file_index}{delimiter}\0",
            printer_item = printer_item,
            tags = item.tags,
            comment = item.comment,
            delimiter = DELIMITER,
            snippet = &item.snippet.trim_end_matches(LINE_SEPARATOR),
            file_index = item.file_index.unwrap_or(0),
        )
}

pub fn read(raw_snippet: &str, is_single: bool) -> Result<(&str, Item)> {
    let mut lines = raw_snippet.split('\0');
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
        .split(DELIMITER)
        .skip(1);

    let tags = parts.next().unwrap_or("").into();
    let comment = parts.next().unwrap_or("").into();
    let snippet = parts.next().unwrap_or("").into();
    let file_index = parts.next().unwrap_or("").parse().ok();

    let item = Item {
        tags,
        comment,
        snippet,
        file_index,
        ..Default::default()
    };

    Ok((key, item))
}
