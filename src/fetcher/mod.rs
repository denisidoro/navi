use crate::structures::item::Item;
use regex::Regex;
use crate::display::Writer;
use crate::parser;
use crate::structures::cheat::VariableMap;
use crate::structures::config::Config;
use crate::common::filesystem::InvalidPath;
use crate::common::filesystem::UnreadableDir;
use crate::welcome;
use anyhow::{Context, Error};
use core::fmt::Display;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub mod filesystem;

pub trait Fetcher {
    fn fetch(self: &Self, config: &Config, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<VariableMap, Error>;
}