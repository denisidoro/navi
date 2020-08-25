use crate::structures::config::Config;
use crate::structures::error::filesystem::InvalidPath;
use crate::structures::error::filesystem::UnreadableDir;
use anyhow::{Context, Error};
use core::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use crate::display::Writer;
use crate::structures::cheat::VariableMap;
use crate::welcome;
use crate::parser;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> Result<impl Iterator<Item = Result<String, Error>>, Error>
where
    P: AsRef<Path> + Display + Copy,
{
    let file = File::open(filename).with_context(|| format!("Failed to open file {}", filename))?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.map_err(Error::from)))
}

pub fn pathbuf_to_string(pathbuf: PathBuf) -> Result<String, Error> {
    Ok(pathbuf
        .as_os_str()
        .to_str()
        .ok_or_else(|| InvalidPath(pathbuf.to_path_buf()))
        .map(str::to_string)?)
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

fn follow_symlink(pathbuf: PathBuf) -> Result<PathBuf, Error> {
    fs::read_link(pathbuf.clone())
        .map(|o| {
            let o_str = o
                .as_os_str()
                .to_str()
                .ok_or_else(|| InvalidPath(o.to_path_buf()))?;
            if o_str.starts_with('.') {
                let parent = pathbuf
                    .parent()
                    .ok_or_else(|| anyhow!("`{}` has no parent", pathbuf.display()))?;
                let parent_str = parent
                    .as_os_str()
                    .to_str()
                    .ok_or_else(|| InvalidPath(parent.to_path_buf()))?;
                let path_str = format!("{}/{}", parent_str, o_str);
                let p = PathBuf::from(path_str);
                follow_symlink(p)
            } else {
                follow_symlink(o)
            }
        })
        .unwrap_or(Ok(pathbuf))
}

fn exe_pathbuf() -> Result<PathBuf, Error> {
    let pathbuf = std::env::current_exe().context("Unable to acquire executable's path")?;
    follow_symlink(pathbuf)
}

pub fn exe_string() -> Result<String, Error> {
    pathbuf_to_string(exe_pathbuf()?)
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

fn cheat_paths(config: &Config) -> Result<String, Error> {
    config
        .path
        .clone()
        .ok_or_else(|| anyhow!("No cheat paths"))
        .or_else(|_| {
            cheat_paths_from_config_dir().context("No directory for cheats in user data directory")
        })
}

pub fn create_dir(path: &str) -> Result<(), Error> {
    fs::create_dir_all(path).with_context(|| format!("Failed to create directory `{}`", path))
}

pub fn remove_dir(path: &str) -> Result<(), Error> {
    fs::remove_dir_all(path).with_context(|| format!("Failed to remove directory `{}`", path))
}

pub fn tmp_path_str() -> Result<String, Error> {
    let cheat_path_str = pathbuf_to_string(cheat_pathbuf()?)?;
    Ok(format!("{}/tmp", cheat_path_str))
}

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

pub fn read_all(
    config: &Config,
    stdin: &mut std::process::ChildStdin,
    writer: &mut dyn Writer,
) -> Result<VariableMap, Error> {
    let mut variables = VariableMap::new();
    let mut found_something = false;
    let mut visited_lines = HashSet::new();
    let paths = cheat_paths(config);

    // TODO: remove
    // read_lines(tldr::markdown_lines(), "markdown", &mut variables, &mut visited_lines, writer, stdin)?;
    // return Ok(variables);

    if paths.is_err() {
        welcome::cheatsheet(writer, stdin);
        return Ok(variables);
    }

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
        welcome::cheatsheet(writer, stdin);
    }

    Ok(variables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_line() {
        let (variable, command, command_options) =
            parse_variable_line("$ user : echo -e \"$(whoami)\\nroot\" --- --prevent-extra")
                .unwrap();
        assert_eq!(command, " echo -e \"$(whoami)\\nroot\" ");
        assert_eq!(variable, "user");
        assert_eq!(
            command_options,
            Some(FinderOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleSelection,
                ..Default::default()
            })
        );
    }
    use std::process::{Command, Stdio};

    #[test]
    fn test_read_file() {
        let path = "tests/cheats/ssh.cheat";
        let mut variables = VariableMap::new();
        let mut child = Command::new("cat")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        let child_stdin = child.stdin.as_mut().unwrap();
        let mut visited_lines: HashSet<u64> = HashSet::new();
        let mut writer: Box<dyn Writer> = Box::new(display::terminal::Writer::new());
        read_file(
            path,
            &mut variables,
            &mut visited_lines,
            &mut *writer,
            child_stdin,
        )
        .unwrap();
        let expected_suggestion = (
            r#" echo -e "$(whoami)\nroot" "#.to_string(),
            Some(FinderOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleSelection,
                ..Default::default()
            }),
        );
        let actual_suggestion = variables.get_suggestion("ssh", "user");
        assert_eq!(Some(&expected_suggestion), actual_suggestion);
    }

    #[test]
    fn splitting_of_dirs_param_may_not_contain_empty_items() {
        // Trailing colon indicates potential extra path. Split returns an empty item for it. This empty item should be filtered away, which is what this test checks.
        let given_path_config = "SOME_PATH:ANOTHER_PATH:";

        let found_paths = paths_from_path_param(given_path_config);

        let mut expected_paths = vec!["SOME_PATH", "ANOTHER_PATH"].into_iter();

        for found in found_paths {
            let expected = expected_paths.next().unwrap();
            assert_eq!(found, expected)
        }
    }
}
