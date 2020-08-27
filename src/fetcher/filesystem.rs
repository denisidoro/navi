use crate::common::filesystem::{pathbuf_to_string, read_lines, InvalidPath, UnreadableDir};
use crate::display::Writer;
use crate::parser;
use crate::structures::cheat::VariableMap;
use crate::structures::config::Config;

use anyhow::{Context, Error};

use std::collections::HashSet;
use std::fs;

use std::io::BufRead;
use std::path::PathBuf;

fn paths_from_path_param<'a>(env_var: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    env_var.split(':').filter(|folder| folder != &"")
}

// TODO: move
fn read_file(
    path: &str,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    writer: &mut dyn Writer,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    let lines = read_lines(path)?;
    parser::read_lines(lines, path, variables, visited_lines, writer, stdin)
}

pub fn cheat_pathbuf() -> Result<PathBuf, Error> {
    dirs::data_dir()
        .map(|mut dir| {
            dir.push("navi");
            dir.push("cheats");
            dir
        })
        .ok_or_else(|| anyhow!("Unable to acquire user data directory for cheatsheets."))
}

fn cheat_paths_from_config_dir() -> Result<String, Error> {
    cheat_pathbuf()
        .and_then(pathbuf_to_string)
        .and_then(|path| {
            fs::read_dir(path.clone())
                .map_err(|e| UnreadableDir::new(path.clone(), e).into())
                .map(|entries| (path, entries))
        })
        .and_then(|(path, dir_entries)| {
            let mut paths_str = String::from("");
            for entry in dir_entries {
                let path = entry.map_err(|e| UnreadableDir::new(path.clone(), e))?;
                paths_str.push_str(
                    path.path()
                        .into_os_string()
                        .to_str()
                        .ok_or_else(|| InvalidPath(path.path()))?,
                );
                paths_str.push_str(":");
            }
            Ok(paths_str)
        })
}

pub fn cheat_paths(path: Option<String>) -> Result<String, Error> {
        path
        .clone()
        .ok_or_else(|| anyhow!("No cheat paths"))
        .or_else(|_| {
            cheat_paths_from_config_dir().context("No directory for cheats in user data directory")
        })
}

pub fn read_all(
    path: Option<String>,
    stdin: &mut std::process::ChildStdin,
    writer: &mut dyn Writer,
) -> Result<Option<VariableMap>, Error> {
    let mut variables = VariableMap::new();
    let mut found_something = false;
    let mut visited_lines = HashSet::new();
    let paths = cheat_paths(path);

    // TODO: remove
    // read_lines(tldr::markdown_lines(), "markdown", &mut variables, &mut visited_lines, writer, stdin)?;
    // return Ok(variables);

    if paths.is_err() {
        return Ok(None);
    };

    let paths = paths.expect("Unable to get paths");
    let folders = paths_from_path_param(&paths);

    for folder in folders {
        if let Ok(dir_entries) = fs::read_dir(folder) {
            for entry in dir_entries {
                if entry.is_ok() {
                    let path = entry.expect("Impossible to read an invalid entry").path();
                    let path_str = path
                        .to_str()
                        .ok_or_else(|| InvalidPath(path.to_path_buf()))?;
                    if path_str.ends_with(".cheat")
                        && read_file(path_str, &mut variables, &mut visited_lines, writer, stdin)
                            .is_ok()
                        && !found_something
                    {
                        found_something = true;
                    }
                }
            }
        }
    }

    if !found_something {
        return Ok(None);
    }

    Ok(Some(variables))
}
