use crate::env_var;
use crate::finder;
use crate::structures::item::Item;
use crate::terminal;
pub use crate::terminal::style::style;
use crate::terminal::style::Color;
use crate::writer;
use std::cmp::max;
use std::collections::HashSet;
use std::iter;

fn parse_ansi(varname: &str, default: Color) -> Color {
    let value: Option<String> = env_var::parse(varname);
    if let Some(v) = value {
        if let Some(a) = terminal::parse_ansi(&v) {
            return a;
        }
    }
    default
}

lazy_static! {
    pub static ref TAG_COLOR: Color = parse_ansi(env_var::TAG_COLOR, Color::Cyan);
    pub static ref COMMENT_COLOR: Color = parse_ansi(env_var::COMMENT_COLOR, Color::Blue);
    pub static ref SNIPPET_COLOR: Color = parse_ansi(env_var::SNIPPET_COLOR, Color::White);
    pub static ref TAG_WIDTH_PERCENTAGE: u16 = env_var::parse(env_var::TAG_WIDTH).unwrap_or(26);
    pub static ref COMMENT_WIDTH_PERCENTAGE: u16 = env_var::parse(env_var::COMMENT_WIDTH).unwrap_or(42);
}

pub fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width = max(20, width * *TAG_WIDTH_PERCENTAGE / 100);
    let comment_width = max(45, width * *COMMENT_WIDTH_PERCENTAGE / 100);
    (usize::from(tag_width), usize::from(comment_width))
}
