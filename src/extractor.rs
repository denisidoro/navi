use crate::cheatsh;
use crate::clipboard;
use crate::display;
use crate::env_vars;
use crate::fetcher::Fetcher;
use crate::filesystem;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::Finder;
use crate::shell::{BashSpawnError, IS_FISH};
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::config::Action;
use crate::structures::config::Config;
use crate::structures::config::Source;
use crate::tldr;
use crate::welcome;
use anyhow::Context;
use anyhow::Error;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub type Output<'a> = (&'a str, &'a str, &'a str, &'a str, Option<usize>);

pub fn extract_from_selections(raw_snippet: &str, is_single: bool) -> Result<Output, Error> {
    let mut lines = raw_snippet.split('\n');
    let key = if is_single {
        "enter"
    } else {
        lines
            .next()
            .context("Key was promised but not present in `selections`")?
    };

    let mut parts = lines
        .next()
        .context("No more parts in `selections`")?
        .split(display::DELIMITER)
        .skip(3);

    let tags = parts.next().unwrap_or("");
    let comment = parts.next().unwrap_or("");
    let snippet = parts.next().unwrap_or("");
    let file_index = parts.next().unwrap_or("").parse().ok();
    Ok((key, tags, comment, snippet, file_index))
}
