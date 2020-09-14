use crate::common::filesystem::{pathbuf_to_string, read_lines, InvalidPath, UnreadableDir};
use crate::display::Writer;
use crate::parser;
use crate::structures::cheat::VariableMap;
use anyhow::{Context, Error};
use directories_next::BaseDirs;
use std::collections::HashSet;
use std::fs;
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
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("Unable to get base dirs"))?;

    let mut pathbuf = PathBuf::from(base_dirs.data_dir());
    pathbuf.push("navi");
    pathbuf.push("cheats");
    Ok(pathbuf)
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
                paths_str.push_str(path.path().into_os_string().to_str().ok_or_else(|| InvalidPath(path.path()))?);
                paths_str.push_str(":");
            }
            Ok(paths_str)
        })
}

pub fn cheat_paths(path: Option<String>) -> Result<String, Error> {
    path.ok_or_else(|| anyhow!("No cheat paths"))
        .or_else(|_| cheat_paths_from_config_dir().context("No directory for cheats in user data directory"))
}

pub fn read_all(path: Option<String>, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error> {
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
                    let path_str = path.to_str().ok_or_else(|| InvalidPath(path.to_path_buf()))?;
                    if path_str.ends_with(".cheat")
                        && read_file(path_str, &mut variables, &mut visited_lines, writer, stdin).is_ok()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display;
    use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
    use std::process::{Command, Stdio};

    #[test]
    fn test_read_file() {
        let path = "tests/cheats/ssh.cheat";
        let mut variables = VariableMap::new();
        let mut child = Command::new("cat").stdin(Stdio::piped()).stdout(Stdio::null()).spawn().unwrap();
        let child_stdin = child.stdin.as_mut().unwrap();
        let mut visited_lines: HashSet<u64> = HashSet::new();
        let mut writer: Box<dyn Writer> = Box::new(display::terminal::Writer::new());
        read_file(path, &mut variables, &mut visited_lines, &mut *writer, child_stdin).unwrap();
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
