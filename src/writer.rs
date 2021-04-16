use crate::config::CONFIG;
use crate::structures::item::Item;
use crate::ui;
use regex::Regex;

const NEWLINE_ESCAPE_CHAR: char = '\x15';
pub const LINE_SEPARATOR: &str = " \x15 ";
pub const DELIMITER: &str = r"  ⠀";

lazy_static! {
    pub static ref NEWLINE_REGEX: Regex = Regex::new(r"\\\s+").expect("Invalid regex");
    pub static ref VAR_REGEX: Regex = Regex::new(r"<(\w[\w\d\-_]*)>").expect("Invalid regex");
    pub static ref COLUMN_WIDTHS: (usize, usize) = ui::get_widths();
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
    let (tag_width, comment_width) = *COLUMN_WIDTHS;
    format!(
            "{tags_short}{delimiter}{comment_short}{delimiter}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}{file_index}{delimiter}\n",
            tags_short = ui::style(limit_str(&item.tags, tag_width)).with(CONFIG.tag_color()),
            comment_short = ui::style(limit_str(&item.comment, comment_width)).with(CONFIG.comment_color()),
            snippet_short = ui::style(fix_newlines(&item.snippet)).with(CONFIG.snippet_color()),
            tags = item.tags,
            comment = item.comment,
            delimiter = DELIMITER,
            snippet = &item.snippet,
            file_index = item.file_index,
        )
}
