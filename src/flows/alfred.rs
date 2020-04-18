use crate::clipboard;
use crate::display;
use crate::filesystem;
use crate::finder::Finder;
use crate::handler;
use crate::parser;
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use crate::structures::option;
use crate::structures::{error::command::BashSpawnError, option::Config};
use anyhow::Context;
use anyhow::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn main(config: Config) -> Result<(), Error> {

        /*
        let mut command = Command::new("cat");
        let mut child = match child {
            Ok(x) => x,
            Err(_) => {
                eprintln!("error cat");
                process::exit(33)
            }
        };
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("Unable to acquire stdin of fzf"))?;
        */

        let mut child = Command::new("cat").stdin(Stdio::piped()).spawn().unwrap();
        let stdin = child.stdin.as_mut().unwrap();

            parser::read_all(&config, stdin).context(
                "Failed to parse variables intended for finder",
            )?;

            Ok(())
}
