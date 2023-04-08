pub use crate::config::CONFIG; // TODO
pub use dns_common::prelude::*;
pub use regex::Regex;

pub use crate::common::fs::ToStringExt;
pub use std::any::{Any, TypeId};
pub use std::collections::{HashMap, HashSet};
pub use std::convert::{TryFrom, TryInto};
pub use std::fmt::Debug;
pub use std::fs::File;
pub use std::io::{BufRead, BufReader};
pub use std::path::{Path, PathBuf};
pub use std::process::Stdio;
pub use std::str::FromStr;
pub use std::sync::{Arc, Mutex, RwLock};

// pub use crate::common::fs::pathbuf_to_string; // TODO

pub trait Runnable {
    fn run(&self) -> Result<()>;
}
