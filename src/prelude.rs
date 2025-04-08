pub use crate::common::deps::HasDeps;
pub use crate::common::fs::ToStringExt;
pub use crate::config::CONFIG; // TODO
pub use crate::libs::dns_common;
pub use anyhow::{anyhow, Context, Error, Result};
pub use regex::Regex;
pub use serde::de::Deserializer;
pub use serde::ser::Serializer;
pub use serde::{Deserialize, Serialize};
pub use std::any::{Any, TypeId};
pub use std::collections::{HashMap, HashSet};
pub use std::convert::{TryFrom, TryInto};
pub use std::fs::File;
pub use std::io::{BufRead, BufReader};
pub use std::path::{Path, PathBuf};
pub use std::process::Stdio;
pub use std::str::FromStr;
pub use std::sync::{Arc, Mutex, RwLock};
pub use tracing::{self, debug, error, event, info, instrument, span, subscriber, trace, warn};

pub trait Component: Any + AsAny + Send + Sync {}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Runnable {
    fn run(&self) -> Result<()>;
}
