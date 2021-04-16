pub use crate::fs::{
    create_dir, exe_string, pathbuf_to_string, read_lines, remove_dir, InvalidPath, UnreadableDir,
};
use crate::parser;
use crate::structures::cheat::VariableMap;
use crate::structures::fetcher;
use anyhow::Result;
use directories_next::BaseDirs;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn all_cheat_files(path: &Path) -> Vec<String> {
    WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .filter(|e| e.ends_with(".cheat"))
        .collect::<Vec<String>>()
}

fn paths_from_path_param(env_var: &str) -> impl Iterator<Item = &str> {
    env_var.split(':').filter(|folder| folder != &"")
}

pub fn default_cheat_pathbuf() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("Unable to get base dirs"))?;

    let mut pathbuf = PathBuf::from(base_dirs.data_dir());
    pathbuf.push("navi");
    pathbuf.push("cheats");
    Ok(pathbuf)
}

pub fn cheat_paths(path: Option<String>) -> Result<String> {
    if let Some(p) = path {
        Ok(p)
    } else {
        pathbuf_to_string(&default_cheat_pathbuf()?)
    }
}

pub fn tmp_pathbuf() -> Result<PathBuf> {
    let mut root = default_cheat_pathbuf()?;
    root.push("tmp");
    Ok(root)
}

fn without_first(string: &str) -> String {
    string
        .char_indices()
        .next()
        .and_then(|(i, _)| string.get(i + 1..))
        .expect("Should have at least one char")
        .to_string()
}

fn gen_lists(tag_rules: Option<String>) -> (Option<Vec<String>>, Option<Vec<String>>) {
    let mut allowlist = None;
    let mut denylist: Option<Vec<String>> = None;

    if let Some(rules) = tag_rules {
        let words: Vec<_> = rules.split(',').collect();
        allowlist = Some(
            words
                .iter()
                .filter(|w| !w.starts_with('!'))
                .map(|w| w.to_string())
                .collect(),
        );
        denylist = Some(
            words
                .iter()
                .filter(|w| w.starts_with('!'))
                .map(|w| without_first(w))
                .collect(),
        );
    }

    (allowlist, denylist)
}

pub struct Fetcher {
    path: Option<String>,
    allowlist: Option<Vec<String>>,
    denylist: Option<Vec<String>>,
}

impl Fetcher {
    pub fn new(path: Option<String>, tag_rules: Option<String>) -> Self {
        let (allowlist, denylist) = gen_lists(tag_rules);
        Self {
            path,
            allowlist,
            denylist,
        }
    }
}

impl fetcher::Fetcher for Fetcher {
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>> {
        let mut variables = VariableMap::new();
        let mut found_something = false;
        let mut visited_lines = HashSet::new();

        let path = self.path.clone();
        let paths = cheat_paths(path);

        if paths.is_err() {
            return Ok(None);
        };

        let paths = paths.expect("Unable to get paths");
        let folders = paths_from_path_param(&paths);

        for folder in folders {
            let folder_pathbuf = PathBuf::from(folder);
            for file in all_cheat_files(&folder_pathbuf) {
                files.push(file.clone());
                let index = files.len() - 1;
                let read_file_result = {
                    let lines = read_lines(&file)?;
                    parser::read_lines(
                        lines,
                        &file,
                        index,
                        &mut variables,
                        &mut visited_lines,
                        stdin,
                        self.allowlist.as_ref(),
                        self.denylist.as_ref(),
                    )
                };

                if read_file_result.is_ok() && !found_something {
                    found_something = true
                }
            }
        }

        if !found_something {
            return Ok(None);
        }

        Ok(Some(variables))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* TODO

    use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
    use crate::writer;
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
        let mut writer: Box<dyn Writer> = Box::new(writer::terminal::Writer::new());
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
    */

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
