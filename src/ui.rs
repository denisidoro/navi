use crate::config::CONFIG;
use crate::terminal;
pub use crate::terminal::style::style;
use std::cmp::max;

pub fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width_percentage = max(
        CONFIG.tag_min_width(),
        width * CONFIG.tag_width_percentage() / 100,
    );
    let comment_width_percentage = max(
        CONFIG.comment_min_width(),
        width * CONFIG.comment_width_percentage() / 100,
    );
    (
        usize::from(tag_width_percentage),
        usize::from(comment_width_percentage),
    )
}
