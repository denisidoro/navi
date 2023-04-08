pub use crate::config::CONFIG; // TODO
pub use dns_common::prelude::*;
pub use regex::Regex;
pub use std::io::{BufRead, BufReader};
pub use std::process::Stdio;
pub use std::str::FromStr;

pub trait Runnable {
    fn run(&self) -> Result<()>;
}
