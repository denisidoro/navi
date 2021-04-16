use crate::env_var;
use crate::finder;
use crate::structures::item::Item;
use crate::terminal;
use crate::terminal::style::{style, Color};
use crate::ui;
use crate::writer;
use std::cmp::max;
use std::collections::HashSet;
use std::iter;

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", text.chars().take(length - 1).collect::<String>())
    } else {
        format!("{:width$}", text, width = length)
    }
}

pub struct Writer {
    tag_width: usize,
    comment_width: usize,
}

impl Writer {
    pub fn new() -> Writer {
        let (tag_width, comment_width) = get_widths();
        writer::terminal::Writer {
            tag_width,
            comment_width,
        }
    }
}

impl writer::Writer for Writer {
    fn write(&mut self, item: &Item) -> String {
        format!(
            "{tags_short}{delimiter}{comment_short}{delimiter}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}{file_index}{delimiter}\n",
            tags_short = ui::style(limit_str(&item.tags, self.tag_width)).with(*ui::TAG_COLOR),
            comment_short = ui::style(limit_str(&item.comment, self.comment_width)).with(*ui::COMMENT_COLOR),
            snippet_short = ui::style(writer::fix_newlines(&item.snippet)).with(*ui::SNIPPET_COLOR),
            tags = item.tags,
            comment = item.comment,
            delimiter = writer::DELIMITER,
            snippet = &item.snippet,
            file_index = item.file_index,
        )
    }
}
