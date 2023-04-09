use crate::prelude::*;
use unicode_width::UnicodeWidthStr;

pub mod raycast;
pub mod terminal;

const NEWLINE_ESCAPE_CHAR: char = '\x15';
pub const LINE_SEPARATOR: &str = " \x15 ";

lazy_static! {
    pub static ref NEWLINE_REGEX: Regex = Regex::new(r"\\\s+").expect("Invalid regex");
    pub static ref VAR_REGEX: Regex = Regex::new(r"\\?<(\w[\w\d\-_]*)>").expect("Invalid regex");
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
    let len = UnicodeWidthStr::width(text);
    if len <= length {
        format!("{}{}", text, " ".repeat(length - len))
    } else {
        let mut new_length = length;
        let mut actual_length = 9999;
        let mut txt = text.to_owned();
        while actual_length >= length {
            txt = txt.chars().take(new_length - 1).collect::<String>();
            actual_length = UnicodeWidthStr::width(txt.as_str());
            new_length -= 1;
        }
        format!("{}…{}", txt, " ".repeat(length - actual_length - 1))
    }
}
