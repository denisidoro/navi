pub use crate::common::fs::{create_dir, exe_string, read_lines, remove_dir};
use crate::env_var;
use crate::parser::Parser;
use crate::prelude::*;

use crate::structures::fetcher;
use etcetera::BaseStrategy;
use regex::Regex;

use std::cell::RefCell;
use std::path;
use std::path::MAIN_SEPARATOR;

use crate::common::git;
use walkdir::WalkDir;

/// Multiple paths are joint by a platform-specific separator.
/// FIXME: it's actually incorrect to assume a path doesn't containing this separator
#[cfg(target_family = "windows")]
pub const JOIN_SEPARATOR: &str = ";";
#[cfg(not(target_family = "windows"))]
pub const JOIN_SEPARATOR: &str = ":";

/// Takes a Path and returns a collection of Strings representing the cheat files contained in a directory.\
/// A cheat file is defined with the `.cheat` or `.cheat.md` extension.
///
/// # Returns
///
/// A `Vec<String>` which contains all the cheat files under the given directory.\
/// Each cheat file entry inside the `Vec` container is a `String` representation of the path of the file
/// and is written in its absolute format, it is not a relative path, like so:
///
/// ```
/// "/home/user/folder/cheat_file.cheat"
/// ```
///
pub fn all_cheat_files(path: &Path) -> Vec<String> {
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        // We make sure it is indeed a file and not a directory
        // There is nothing that forbids a directory to end with a `.cheat` or `.cheat.md` name.
        .map(|e| if e.path().is_file() {
            e.path().to_str().unwrap_or("").to_string()
        } else {
            "".to_string()
        })
        .filter(|e| e.ends_with(".cheat") || e.ends_with(".cheat.md"))
        .collect::<Vec<String>>()
}

/// Takes a Path and returns a collection of Strings representing the git files contained in a directory.\
/// A git file is defined by being under the `.git` folder.
///
/// # Returns
///
/// A `Vec<String>` which contains all the git files under the given directory.\
/// Each git file entry inside the `Vec` container is a `String` representation of the path of the file
/// and is written in its absolute format, it is not a relative path, like so:
///
/// ```
/// "/home/user/folder/.git/<git-file>"
/// ```
///
pub fn all_git_files(path: &Path) -> Vec<String> {
    let mut path_str = path.to_str().unwrap().to_owned();

    if path_str.ends_with(path::MAIN_SEPARATOR_STR) {
        // We're removing the trailing '/' at the end, if it exists
        path_str.push(path::MAIN_SEPARATOR);
    }

    let git_filter = format!("{}.git{}", path::MAIN_SEPARATOR_STR, path::MAIN_SEPARATOR_STR);

    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| {
            return if e.path().is_file() {
                e.path()
                    .to_str()
                    .unwrap()
                    .replace(path_str.as_str(), "")
                    .to_string()
            } else {
                "".to_string()
            };
        })
        .filter(|e| e.contains(git_filter.as_str()))
        .collect::<Vec<String>>()
}

fn paths_from_path_param(env_var: &str) -> impl Iterator<Item = &str> {
    env_var.split(JOIN_SEPARATOR).filter(|folder| folder != &"")
}

fn compiled_default_path(path: Option<&str>) -> Option<PathBuf> {
    match path {
        Some(path) => {
            let path = if path.contains(MAIN_SEPARATOR) {
                path.split(MAIN_SEPARATOR).next().unwrap()
            } else {
                path
            };
            let path = Path::new(path);
            if path.exists() {
                Some(path.to_path_buf())
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn default_cheat_pathbuf() -> Result<PathBuf> {
    let mut pathbuf = get_data_dir_by_platform()?;

    pathbuf.push("navi");
    pathbuf.push("cheats");

    if pathbuf.exists() {
        if let Some(path) = compiled_default_path(option_env!("NAVI_PATH")) {
            pathbuf = path;
        }
    }
    Ok(pathbuf)
}

pub fn default_config_pathbuf() -> Result<PathBuf> {
    let mut pathbuf = get_config_dir_by_platform()?;

    pathbuf.push("navi");
    pathbuf.push("config.yaml");

    if !pathbuf.exists() {
        if let Some(path) = compiled_default_path(option_env!("NAVI_CONFIG")) {
            pathbuf = path;
        }
    }
    Ok(pathbuf)
}

pub fn cheat_paths(path: Option<String>) -> Result<String> {
    if let Some(p) = path {
        Ok(p)
    } else {
        Ok(default_cheat_pathbuf()?.to_string())
    }
}

/// Returns the cheats path defined at run time
pub fn running_cheats_path() -> String {
    CONFIG.path().unwrap_or_else(|| {
        // if we don't have a path, use the default value
        let _cheats = default_cheat_pathbuf().unwrap();

        _cheats.to_string()
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//
// Here are other functions, unrelated to CLI commands (or at least not directly related)
//
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Returns the data dir computed for each platform.
///
/// We are currently handling two cases: When the platform is `macOS` and when the platform isn't (including `Windows` and `Linux/Unix` platforms)
fn get_data_dir_by_platform() -> Result<PathBuf> {
    if cfg!(target_os = "macos") {
        let base_dirs = etcetera::base_strategy::Apple::new()?;

        Ok(base_dirs.data_dir())
    } else {
        let base_dirs = etcetera::choose_base_strategy()?;

        Ok(base_dirs.data_dir())
    }
}

/// Returns the config dir computed for each platform.
///
/// We are currently handling two cases: When the platform is `macOS` and when the platform isn't (including `Windows` and `Linux/Unix` platforms)
fn get_config_dir_by_platform() -> Result<PathBuf> {
    if cfg!(target_os = "macos") {
        let base_dirs = etcetera::base_strategy::Apple::new()?;

        Ok(base_dirs.config_dir())
    } else {
        let base_dirs = etcetera::choose_base_strategy()?;

        Ok(base_dirs.config_dir())
    }
}

/// Goes through the cheats path(s) defined within the configuration
/// and sends back any cheatsheet repository remote URI it finds along its local URI.
///
/// # Returns
///
/// It returns a Tuple containing two `Vec<String>`, one for the remote URI and one for the local URI.\
/// In a way, you could simplify this function by the following definition:
///
/// ```
/// pub fn local_cheatsheet_repositories() -> (Vec<String>, Vec<String>) {
///     let cheats_repos_remote_uris: Vec<String> = Vec::new();
///     let cheats_repos_local_path: Vec<String> = Vec::new();
///
///     ..
///     return (cheats_repos_remote_uris, cheats_repos_local_path)
/// }
/// ```
///
/// # Examples
///
/// - If there is at least one cheatsheet repository on the filesystem added to navi:
///
///     ```
///     use crate::filesystem::local_cheatsheet_repositories;
///
///     let (cheats_uri, cheats_path): (Vec<String>, Vec<String>) = local_cheatsheet_repositories();
///
///     assert_ne!([], cheats_uri);
///     assert_ne!([], cheats_path);
///     ```
///
pub fn local_cheatsheet_repositories() -> (Vec<String>, Vec<String>) {
    let mut cheats_repos_uri: Vec<String> = Vec::new();
    let mut cheats_repos_paths: Vec<String> = Vec::new();
    let cheats = running_cheats_path();

    // We're checking each given paths possible
    for cheat_path in cheats.split(':') {
        // If the path doesn't exist, continue to the next one
        if !std::fs::exists(cheat_path).unwrap() {
            continue;
        }

        let curr_dir = std::fs::read_dir(cheat_path).unwrap();

        // We're checking subfolders -> they should contain at least one .cheat files
        for entry in curr_dir {
            let entry = entry.unwrap();

            if entry.file_type().unwrap().is_dir() {
                // If the directory doesn't have at least one cheat file -> ignore it and continue
                if all_cheat_files(&entry.path()).is_empty() {
                    continue;
                };

                // If the directory have at least one cheat file -> add it to the list
                // Note: for the list, we are registering the git remote name and not the
                //      folder name since we modify it internally.
                let git_path = format!("{}/{}", &entry.path().display(), ".git");
                let folder_path = entry.path().display().to_string();

                if std::fs::exists(&git_path).unwrap() {
                    let remote_uri = git::get_remote(&entry.path().to_string()).unwrap();

                    cheats_repos_uri.push(remote_uri);
                } else {
                    cheats_repos_uri.push(folder_path.clone());
                }

                cheats_repos_paths.push(folder_path);
            }
        }
    }

    (cheats_repos_uri, cheats_repos_paths)
}

pub fn tmp_pathbuf() -> Result<PathBuf> {
    let mut root = default_cheat_pathbuf()?;
    root.push("tmp");
    Ok(root)
}

fn interpolate_paths(paths: String) -> String {
    let re = Regex::new(r#"\$\{?[a-zA-Z_][a-zA-Z_0-9]*"#).unwrap();
    let mut newtext = paths.to_string();
    for capture in re.captures_iter(&paths) {
        if let Some(c) = capture.get(0) {
            let varname = c.as_str().replace(['$', '{', '}'], "");
            if let Ok(replacement) = &env_var::get(&varname) {
                newtext = newtext
                    .replace(&format!("${varname}"), replacement)
                    .replace(&format!("${{{varname}}}"), replacement);
            }
        }
    }
    newtext
}

#[derive(Debug)]
pub struct Fetcher {
    path: Option<String>,
    files: RefCell<Vec<String>>,
}

impl Fetcher {
    pub fn new(path: Option<String>) -> Self {
        Self {
            path,
            files: Default::default(),
        }
    }
}

impl fetcher::Fetcher for Fetcher {
    fn fetch(&self, parser: &mut Parser) -> Result<bool> {
        let mut found_something = false;

        let path = self.path.clone();
        let paths = cheat_paths(path);

        if paths.is_err() {
            return Ok(false);
        };

        let paths = paths.expect("Unable to get paths");
        let interpolated_paths = interpolate_paths(paths);
        let folders = paths_from_path_param(&interpolated_paths);

        let home_regex = Regex::new(r"^~").unwrap();
        let home = etcetera::home_dir().ok();

        // parser.filter = self.tag_rules.as_ref().map(|r| gen_lists(r.as_str()));

        for folder in folders {
            let interpolated_folder = match &home {
                Some(h) => home_regex.replace(folder, h.to_string_lossy()).to_string(),
                None => folder.to_string(),
            };
            let folder_pathbuf = PathBuf::from(interpolated_folder);
            let cheat_files = all_cheat_files(&folder_pathbuf);
            debug!("read cheat files in `{folder_pathbuf:?}`: {cheat_files:#?}");
            for file in cheat_files {
                self.files.borrow_mut().push(file.clone());
                let index = self.files.borrow().len() - 1;
                let read_file_result = {
                    let path = PathBuf::from(&file);
                    let lines = read_lines(&path)?;
                    parser.read_lines(lines, &file, Some(index))
                };

                if read_file_result.is_ok() && !found_something {
                    found_something = true
                }
            }
        }

        debug!("FilesystemFetcher = {self:#?}");
        Ok(found_something)
    }

    fn files(&self) -> Vec<String> {
        self.files.borrow().clone()
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

    #[test]
    fn test_default_config_pathbuf() {
        let base_dirs = etcetera::choose_base_strategy().expect("could not determine base directories");

        let expected = {
            let mut e = base_dirs.config_dir();
            e.push("navi");
            e.push("config.yaml");
            e.to_string_lossy().to_string()
        };

        let config = default_config_pathbuf().expect("could not find default config path");

        assert_eq!(expected, config.to_string_lossy().to_string())
    }

    #[test]
    fn test_default_cheat_pathbuf() {
        let base_dirs = etcetera::choose_base_strategy().expect("could not determine base directories");

        let expected = {
            let mut e = base_dirs.data_dir();
            e.push("navi");
            e.push("cheats");
            e.to_string_lossy().to_string()
        };

        let cheats = default_cheat_pathbuf().expect("could not find default config path");

        assert_eq!(expected, cheats.to_string_lossy().to_string())
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn multiple_paths() {
        let p = r#"C:\Users\Administrator\AppData\Roaming\navi\config.yaml"#;
        let paths = &[p; 2].join(JOIN_SEPARATOR);
        assert_eq!(paths_from_path_param(paths).collect::<Vec<_>>(), [p; 2]);
    }
}
