use crate::terminal;
use regex::Regex;
use std::cmp::max;
use termion::color;
use crate::structures::item::Item;

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

pub fn format_line(
    tags: &str,
    comment: &str,
    snippet: &str,
    tag_width: usize,
    comment_width: usize,
) -> String {
    format!(
       "{tag_color}{tags_short}{delimiter}{comment_color}{comment_short}{delimiter}{snippet_color}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}\n",
       tags_short = limit_str(tags, tag_width),
       comment_short = limit_str(comment, comment_width),
       snippet_short = fix_newlines(snippet),
       comment_color = color::Fg(COMMENT_COLOR),
       tag_color = color::Fg(TAG_COLOR),
       snippet_color = color::Fg(SNIPPET_COLOR),
       tags = tags,
       comment = comment,
       delimiter = DELIMITER,
       snippet = &snippet)
}

pub trait Writer {
    fn write(&mut self, item: Item) -> String;
}

pub struct FinderWriter {
    pub tag_width: usize,
    pub comment_width: usize
}

pub struct AlfredWriter {
    pub is_first: bool
}

impl Writer for FinderWriter {
    fn write(&mut self, item: Item) -> String {
        format_line(item.tags, item.comment, item.snippet, self.tag_width, self.comment_width)
    }
}

impl Writer for AlfredWriter {
    fn write(&mut self, item: Item) -> String {
        let prefix = if self.is_first == true {
            self.is_first = false;
            ""
        } else {
            ","
        };

        let tags = item.tags.replace('"', "").replace('\\', "").replace(NEWLINE_ESCAPE_CHAR, " ");
        let comment = item.comment.replace('"', "").replace('\\', "").replace(NEWLINE_ESCAPE_CHAR, " ");
        let snippet = item.snippet.replace('"', "").replace('\\', "").replace(NEWLINE_ESCAPE_CHAR, " ");

        format!(r#"{prefix}{{"type":"file","title":"{comment}","match":"{comment} {tags} {snippet}","subtitle":"subtitle","variables":{{"tag":"mytag","comment":"mycomment","snippet":"navi fn url::open https://google.com/?q=<foo>+<bar>"}},"autocomplete":"Desktop","icon":{{"type":"fileicon","path":"~/Desktop"}}}}"#, 
        prefix = prefix, 
        tags = tags,
        comment = comment,
        snippet = snippet)

    }
}
