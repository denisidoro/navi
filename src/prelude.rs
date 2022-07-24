pub use crate::common::prelude::*;
pub use crate::config::CONFIG; // TODO
pub use regex::Regex;

// pub use crate::fs::pathbuf_to_string; // TODO

pub trait Runnable {
    fn run(&self) -> Result<()>;
}
