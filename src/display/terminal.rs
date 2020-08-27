use crate::common::terminal_width;
use crate::display;
use crate::structures::item::Item;
use std::cmp::max;
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

fn get_widths() -> (usize, usize) {
    let width = terminal_width::get();
    let tag_width = max(4, width * 20 / 100);
    let comment_width = max(4, width * 40 / 100);
    (usize::from(tag_width), usize::from(comment_width))
}

pub struct Writer {
    tag_width: usize,
    comment_width: usize,
}

impl Writer {
    pub fn new() -> Writer {
        let (tag_width, comment_width) = get_widths();
        display::terminal::Writer {
            tag_width,
            comment_width,
        }
    }
}

impl display::Writer for Writer {
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
