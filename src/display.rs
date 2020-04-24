use crate::structures::item::Item;
use crate::terminal;
use regex::Regex;
use std::cmp::max;
use termion::color;

const COMMENT_COLOR: color::LightCyan = color::LightCyan;
const TAG_COLOR: color::Blue = color::Blue;
const SNIPPET_COLOR: color::White = color::White;

const NEWLINE_ESCAPE_CHAR: char = '\x15';
pub const LINE_SEPARATOR: &str = " \x15 ";
pub const DELIMITER: &str = r"  ⠀";

lazy_static! {
    pub static ref NEWLINE_REGEX: Regex = Regex::new(r"\\\s+").expect("Invalid regex");
}

pub fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width = max(4, width * 20 / 100);
    let comment_width = max(4, width * 40 / 100);
    (usize::from(tag_width), usize::from(comment_width))
}

pub fn variable_prompt(varname: &str) -> String {
    format!("{}: ", varname)
}

fn fix_newlines(txt: &str) -> String {
    if txt.contains(NEWLINE_ESCAPE_CHAR) {
        (*NEWLINE_REGEX)
            .replace_all(txt.replace(LINE_SEPARATOR, "  ").as_str(), "")
            .to_string()
    } else {
        txt.to_string()
    }
}

pub fn preview(comment: &str, tags: &str, snippet: &str) {
    println!(
        "{comment_color}{comment} {tag_color}{tags} \n{snippet_color}{snippet}",
        comment = format!("# {}", comment),
        tags = format!("[{}]", tags),
        snippet = fix_newlines(snippet),
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

pub trait Writer {
    fn write(&mut self, item: Item) -> String;
}

pub struct FinderWriter {
    pub tag_width: usize,
    pub comment_width: usize,
}

pub struct AlfredWriter {
    pub is_first: bool,
}

impl Writer for FinderWriter {
    fn write(&mut self, item: Item) -> String {
        format!(
       "{tag_color}{tags_short}{delimiter}{comment_color}{comment_short}{delimiter}{snippet_color}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}\n",
       tags_short = limit_str(item.tags, self.tag_width),
       comment_short = limit_str(item.comment, self.comment_width),
       snippet_short = fix_newlines(item.snippet),
       comment_color = color::Fg(COMMENT_COLOR),
       tag_color = color::Fg(TAG_COLOR),
       snippet_color = color::Fg(SNIPPET_COLOR),
       tags = item.tags,
       comment = item.comment,
       delimiter = DELIMITER,
       snippet = &item.snippet)
    }
}

fn escape_for_json(txt: &str) -> String {
    txt.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(NEWLINE_ESCAPE_CHAR, " ")
}

impl Writer for AlfredWriter {
    fn write(&mut self, item: Item) -> String {
        let prefix = if self.is_first {
            self.is_first = false;
            ""
        } else {
            ","
        };

        let tags = escape_for_json(item.tags);
        let comment = escape_for_json(item.comment);
        let snippet = escape_for_json(item.snippet);

        format!(
            r#"{prefix}{{"type":"file","title":"{comment}","match":"{comment} {tags} {snippet}","subtitle":"{tags} :: {snippet}","variables":{{"tags":"{tags}","comment":"{comment}","snippet":"{snippet}"}},"icon":{{"path":"navi.png"}}}}"#,
            prefix = prefix,
            tags = tags,
            comment = comment,
            snippet = snippet
        )
    }
}
