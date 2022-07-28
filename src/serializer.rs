use crate::common::terminal;
use crate::prelude::*;
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

const NEWLINE_ESCAPE_CHAR: char = '\x15';
const FIELD_SEP_ESCAPE_CHAR: char = '\x16';
pub const LINE_SEPARATOR: &str = " \x15 ";
pub const DELIMITER: &str = r"  ⠀";

lazy_static! {
    pub static ref NEWLINE_REGEX: Regex = Regex::new(r"\\\s+").expect("Invalid regex");
    pub static ref VAR_REGEX: Regex = Regex::new(r"\\?<(\w[\w\d\-_]*)>").expect("Invalid regex");
    pub static ref COLUMN_WIDTHS: (usize, usize, usize) = get_widths();
}

pub fn with_new_lines(txt: String) -> String {
    txt.replace(LINE_SEPARATOR, "\n")
}

pub fn fix_newlines(txt: &str) -> String {
    if txt.contains(NEWLINE_ESCAPE_CHAR) {
        (*NEWLINE_REGEX)
            .replace_all(txt.replace(LINE_SEPARATOR, "  ").as_str(), "")
            .to_string()
    } else {
        txt.to_string()
    }
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", text.chars().take(length - 1).collect::<String>())
    } else {
        format!("{:width$}", text, width = length)
    }
}

pub fn write(item: &Item) -> String {
    let (tag_width_percentage, comment_width_percentage, snippet_width_percentage) = *COLUMN_WIDTHS;
    format!(
            "{tags_short}{delimiter}{comment_short}{delimiter}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}{file_index}{delimiter}\n",
            tags_short = style(limit_str(&item.tags, tag_width_percentage)).with(CONFIG.tag_color()),
            comment_short = style(limit_str(&item.comment, comment_width_percentage)).with(CONFIG.comment_color()),
            snippet_short = style(limit_str(&fix_newlines(&item.snippet), snippet_width_percentage)).with(CONFIG.snippet_color()),
            tags = item.tags,
            comment = item.comment,
            delimiter = DELIMITER,
            snippet = &item.snippet.trim_end_matches(LINE_SEPARATOR),
            file_index = item.file_index.unwrap_or(0),
        )
}

pub fn write_raw(item: &Item) -> String {
    format!(
        "{hash}{delimiter}{tags}{delimiter}{comment}{delimiter}{icon}{delimiter}{snippet}\n",
        hash = item.hash(),
        tags = item.tags,
        comment = item.comment,
        delimiter = FIELD_SEP_ESCAPE_CHAR,
        icon = item.icon.clone().unwrap_or_default(),
        snippet = &item.snippet.trim_end_matches(LINE_SEPARATOR),
    )
}

pub fn raycast_deser(line: &str) -> Result<Item> {
    let mut parts = line.split(FIELD_SEP_ESCAPE_CHAR);
    let hash: u64 = parts
        .next()
        .context("no hash")?
        .parse()
        .context("hash not a u64")?;
    let tags = parts.next().context("no tags")?.into();
    let comment = parts.next().context("no comment")?.into();
    let icon_str = parts.next().context("no icon")?;
    let snippet = parts.next().context("no snippet")?.into();

    let icon = if icon_str.is_empty() {
        None
    } else {
        Some(icon_str.into())
    };

    let item = Item {
        tags,
        comment,
        icon,
        snippet,
        ..Default::default()
    };

    if item.hash() != hash {
        dbg!(&item.hash());
        dbg!(hash);
        Err(anyhow!("Incorrect hash"))
    } else {
        Ok(item)
    }
}
