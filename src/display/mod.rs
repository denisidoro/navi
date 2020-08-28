pub mod alfred;
pub mod terminal;

use crate::structures::item::Item;
use regex::Regex;

const NEWLINE_ESCAPE_CHAR: char = '\x15';
pub const LINE_SEPARATOR: &str = " \x15 ";
pub const DELIMITER: &str = r"  ⠀";

lazy_static! {
    pub static ref NEWLINE_REGEX: Regex = Regex::new(r"\\\s+").expect("Invalid regex");
    pub static ref VAR_REGEX: Regex = Regex::new(r"<(\w[\w\d\-_]*)>").expect("Invalid regex");
}

pub fn with_new_lines(txt: String) -> String {
    txt.replace(LINE_SEPARATOR, "\n")
}

pub fn fix_newlines(txt: &str) -> String {
    if txt.contains(NEWLINE_ESCAPE_CHAR) {
        (*NEWLINE_REGEX).replace_all(txt.replace(LINE_SEPARATOR, "  ").as_str(), "").to_string()
    } else {
        txt.to_string()
    }
}

pub trait Writer {
    fn write(&mut self, item: Item) -> String;
}
