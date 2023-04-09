use crate::prelude::*;
pub use env::remove_var as remove;
pub use env::set_var as set;
pub use env::var as get;
use std::env;

pub const PREVIEW_INITIAL_SNIPPET: &str = "NAVI_PREVIEW_INITIAL_SNIPPET";
pub const PREVIEW_TAGS: &str = "NAVI_PREVIEW_TAGS";
pub const PREVIEW_COMMENT: &str = "NAVI_PREVIEW_COMMENT";
pub const PREVIEW_COLUMN: &str = "NAVI_PREVIEW_COLUMN";
pub const PREVIEW_DELIMITER: &str = "NAVI_PREVIEW_DELIMITER";
pub const PREVIEW_MAP: &str = "NAVI_PREVIEW_MAP";

pub const PATH: &str = "NAVI_PATH";
pub const FZF_OVERRIDES: &str = "NAVI_FZF_OVERRIDES";
pub const FZF_OVERRIDES_VAR: &str = "NAVI_FZF_OVERRIDES_VAR";
pub const FINDER: &str = "NAVI_FINDER";

pub const CONFIG: &str = "NAVI_CONFIG";
pub const CONFIG_YAML: &str = "NAVI_CONFIG_YAML";

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
        panic!("{name} not set")
    }
}

pub fn escape(name: &str) -> String {
    name.replace('-', "_")
}
