use crate::display::Writer;
pub use crate::fs::{
    create_dir, exe_string, pathbuf_to_string, read_lines, remove_dir, InvalidPath, UnreadableDir,
};
use crate::parser;
use crate::structures::cheat::VariableMap;
use crate::structures::fetcher;
use anyhow::Error;
use directories_next::BaseDirs;
use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn all_cheat_files(path_str: &str) -> Vec<String> {
    let path_str_with_trailing_slash = if path_str.ends_with('/') {
        path_str.to_string()
    } else {
        format!("{}/", &path_str)
    };

    WalkDir::new(&path_str)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .filter(|e| e.ends_with(".cheat"))
        .map(|e| e.replace(&path_str_with_trailing_slash, ""))
        .collect::<Vec<String>>()
}

fn paths_from_path_param(env_var: &str) -> impl Iterator<Item = &str> {
    env_var.split(':').filter(|folder| folder != &"")
}

// TODO: move
fn read_file(
    path: &str,
    file_index: usize,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    writer: &mut dyn Writer,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    let lines = read_lines(path)?;
    parser::read_lines(lines, path, file_index, variables, visited_lines, writer, stdin)
}

pub fn default_cheat_pathbuf() -> Result<PathBuf, Error> {
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("Unable to get base dirs"))?;

    let mut pathbuf = PathBuf::from(base_dirs.data_dir());
    pathbuf.push("navi");
    pathbuf.push("cheats");
    Ok(pathbuf)
}

pub fn cheat_paths(path: Option<String>) -> Result<String, Error> {
    if let Some(p) = path {
        Ok(p)
    } else {
        pathbuf_to_string(default_cheat_pathbuf()?)
    }
}

pub fn read_all(
    path: Option<String>,
    files: &mut Vec<String>,
    stdin: &mut std::process::ChildStdin,
    writer: &mut dyn Writer,
) -> Result<Option<VariableMap>, Error> {
    let mut variables = VariableMap::new();
    let mut found_something = false;
    let mut visited_lines = HashSet::new();
    let paths = cheat_paths(path);

    if paths.is_err() {
        return Ok(None);
    };

    let paths = paths.expect("Unable to get paths");
    let folders = paths_from_path_param(&paths);

    for folder in folders {
        for file in all_cheat_files(folder) {
            let full_filename = format!("{}/{}", &folder, &file);
            files.push(full_filename.clone());
            let index = files.len() - 1;
            if read_file(
                &full_filename,
                index,
                &mut variables,
                &mut visited_lines,
                writer,
                stdin,
            )
            .is_ok()
                && !found_something
            {
                found_something = true
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
    use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
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
            0,
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

pub fn tmp_path_str() -> Result<String, Error> {
    let cheat_path_str = pathbuf_to_string(default_cheat_pathbuf()?)?;
    Ok(format!("{}/tmp", cheat_path_str))
}

pub struct Fetcher {
    path: Option<String>,
}

impl Fetcher {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl fetcher::Fetcher for Fetcher {
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        writer: &mut dyn Writer,
        files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>, Error> {
        read_all(self.path.clone(), files, stdin, writer)
    }
}
