use crate::display::{self, Writer};
use crate::structures::item::Item;
use crate::terminal_width;
use termion::color;

const COMMENT_COLOR: color::LightCyan = color::LightCyan;
const TAG_COLOR: color::Blue = color::Blue;
const SNIPPET_COLOR: color::White = color::White;

pub fn variable_prompt(varname: &str) -> String {
    format!("{}: ", varname)
}

pub fn preview(comment: &str, tags: &str, snippet: &str) {
    println!(
        "{comment_color}{comment} {tag_color}{tags} \n{snippet_color}{snippet}",
        comment = format!("# {}", comment),
        tags = format!("[{}]", tags),
        snippet = display::fix_newlines(snippet),
        comment_color = color::Fg(COMMENT_COLOR),
        tag_color = color::Fg(TAG_COLOR),
        snippet_color = color::Fg(SNIPPET_COLOR),
    );
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", text.chars().take(length - 1).collect::<String>())
    } else {
        format!("{:width$}", text, width = length)
    }
}

pub struct TerminalWriter {
    tag_width: usize,
    comment_width: usize,
}

pub fn new_writer() -> TerminalWriter {
    let (tag_width, comment_width) = terminal_width::get_widths();
    display::terminal::TerminalWriter {
        tag_width,
        comment_width,
    }
}

impl Writer for TerminalWriter {
    fn write(&mut self, item: Item) -> String {
        format!(
       "{tag_color}{tags_short}{delimiter}{comment_color}{comment_short}{delimiter}{snippet_color}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}\n",
       tags_short = limit_str(item.tags, self.tag_width),
       comment_short = limit_str(item.comment, self.comment_width),
       snippet_short = display::fix_newlines(item.snippet),
       comment_color = color::Fg(COMMENT_COLOR),
       tag_color = color::Fg(TAG_COLOR),
       snippet_color = color::Fg(SNIPPET_COLOR),
       tags = item.tags,
       comment = item.comment,
       delimiter = display::DELIMITER,
       snippet = &item.snippet)
    }
}
