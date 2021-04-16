use crate::config::CONFIG;
use crate::terminal;
pub use crate::terminal::style::style;
use std::cmp::max;

pub fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width = max(CONFIG.tag_min_abs_width(), width * CONFIG.tag_width() / 100);
    let comment_width = max(
        CONFIG.comment_min_abs_width(),
        width * CONFIG.comment_width() / 100,
    );
    (usize::from(tag_width), usize::from(comment_width))
}
