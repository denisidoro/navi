use termion::{color, terminal_size};

pub static COMMENT_COLOR: color::LightCyan = color::LightCyan;
pub static TAG_COLOR: color::Blue = color::Blue;
pub static SNIPPET_COLOR: color::White = color::White;

pub fn widths() -> (usize, usize) {
let full_width = terminal_size().unwrap().0;
let tag_width = full_width*10/100;
let comment_width = full_width*50/100;
(usize::from(tag_width), usize::from(comment_width))
}