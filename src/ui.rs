use crate::env_var;

use crate::terminal;
pub use crate::terminal::style::style;
use crate::terminal::style::Color;
use std::cmp::max;

lazy_static! {
    pub static ref TAG_WIDTH_PERCENTAGE: u16 = env_var::parse(env_var::TAG_WIDTH).unwrap_or(26);
    pub static ref COMMENT_WIDTH_PERCENTAGE: u16 = env_var::parse(env_var::COMMENT_WIDTH).unwrap_or(42);
}

pub fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width = max(20, width * *TAG_WIDTH_PERCENTAGE / 100);
    let comment_width = max(45, width * *COMMENT_WIDTH_PERCENTAGE / 100);
    (usize::from(tag_width), usize::from(comment_width))
}
