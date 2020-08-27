use anyhow::{Context, Error};
use core::fmt::Display;
use std::fs::{self, create_dir_all, remove_dir_all, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid path `{0}`")]
pub struct InvalidPath(pub PathBuf);

#[derive(Error, Debug)]
#[error("Unable to read directory `{dir}`")]
pub struct UnreadableDir {
    dir: PathBuf,
    #[source]
    source: anyhow::Error,
}

impl UnreadableDir {
    pub fn new<DirT, SourceError>(dir: DirT, source: SourceError) -> Self
    where
        DirT: Into<PathBuf>,
        SourceError: std::error::Error + Sync + Send + 'static,
    {
        UnreadableDir {
            dir: dir.into(),
            source: source.into(),
        }
    }
}

pub fn read_lines<P>(filename: P) -> Result<impl Iterator<Item = Result<String, Error>>, Error>
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

pub fn create_dir(path: &str) -> Result<(), Error> {
    create_dir_all(path).with_context(|| format!("Failed to create directory `{}`", path))
}

pub fn remove_dir(path: &str) -> Result<(), Error> {
    remove_dir_all(path).with_context(|| format!("Failed to remove directory `{}`", path))
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
