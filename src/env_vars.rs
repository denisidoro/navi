pub use env::remove_var as remove;
pub use env::set_var as set;
pub use env::var as get;
use std::env;
use std::str::FromStr;

pub const PREVIEW_INITIAL_SNIPPET: &str = "NAVI_PREVIEW_INITIAL_SNIPPET";
pub const PREVIEW_TAGS: &str = "NAVI_PREVIEW_TAGS";
pub const PREVIEW_COMMENT: &str = "NAVI_PREVIEW_COMMENT";
pub const PREVIEW_COLUMN: &str = "NAVI_PREVIEW_COLUMN";
pub const PREVIEW_DELIMITER: &str = "NAVI_PREVIEW_DELIMITER";
pub const PREVIEW_MAP: &str = "NAVI_PREVIEW_MAP";

pub const TAG_COLOR: &str = "NAVI_TAG_COLOR";
pub const COMMENT_COLOR: &str = "NAVI_COMMENT_COLOR";
pub const SNIPPET_COLOR: &str = "NAVI_SNIPPET_COLOR";

pub const TAG_WIDTH: &str = "NAVI_TAG_WIDTH";
pub const COMMENT_WIDTH: &str = "NAVI_COMMENT_WIDTH";

pub const PATH: &str = "NAVI_PATH";
pub const FZF_OVERRIDES: &str = "NAVI_FZF_OVERRIDES";
pub const FZF_OVERRIDES_VAR: &str = "NAVI_FZF_OVERRIDES_VAR";
pub const FINDER: &str = "NAVI_FINDER";

pub fn parse<T: FromStr>(varname: &str) -> Option<T> {
    if let Ok(x) = env::var(varname) {
        x.parse::<T>().ok()
    } else {
        None
    }
}

pub fn must_get(name: &str) -> String {
    if let Ok(v) = env::var(name) {
        v
    } else {
        panic!("{} not set", name)
    }
}

pub fn escape(name: &str) -> String {
    name.replace('-', "_")
}
