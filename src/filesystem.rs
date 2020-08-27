use crate::display::Writer;
use crate::structures::config::Config;
use crate::structures::cheat::VariableMap;
pub use crate::common::filesystem::{InvalidPath, UnreadableDir, pathbuf_to_string, exe_string, remove_dir, create_dir};
use anyhow::{Context, Error};
use core::fmt::Display;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::{fmt::Debug};
use thiserror::Error;
use crate::fetcher::Fetcher;
pub use crate::fetcher::filesystem::{read_all, cheat_pathbuf};

pub fn tmp_path_str() -> Result<String, Error> {
    let cheat_path_str = pathbuf_to_string(cheat_pathbuf()?)?;
    Ok(format!("{}/tmp", cheat_path_str))
}

pub struct Foo {
}

impl Foo {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Fetcher for Foo {
    fn fetch(&self, config: &Config, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<VariableMap, Error> {
      read_all(config, stdin, writer)    
    }
}