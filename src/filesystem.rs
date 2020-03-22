use crate::structures::option::Config;
use anyhow::Context;
use anyhow::Error;
use core::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn read_lines<P>(filename: P) -> Result<Vec<String>, Error>
where
    P: AsRef<Path> + Display,
{
    let error_string = format!("Failed to read lines from `{}`", filename);
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .map(|line| line.map_err(Error::from))
        .collect::<Result<Vec<String>, Error>>()
        .with_context(|| error_string)
}

pub fn pathbuf_to_string(pathbuf: PathBuf) -> Result<String, Error> {
    pathbuf
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow!("Invalid path `{}`", pathbuf.display()))
        .map(str::to_string)
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
                .ok_or_else(|| anyhow!("Invalid path `{}`", o.display()))?;
            if o_str.starts_with('.') {
                let parent_str = pathbuf
                    .parent()
                    .ok_or_else(|| anyhow!("`{}` has no parent", pathbuf.display()))?
                    .as_os_str()
                    .to_str()
                    .ok_or_else(|| {
                        anyhow!("Parent of `{}` is an invalid path", pathbuf.display())
                    })?;
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
                .with_context(|| format!("Unable to read directory `{}`", &path))
                .map(|entries| (path, entries))
        })
        .and_then(|(path, dir_entries)| {
            let mut paths_str = String::from("");
            for entry in dir_entries {
                let path = entry.with_context(|| format!("Unable to read directory `{}`", path))?;
                paths_str.push_str(
                    path.path()
                        .into_os_string()
                        .to_str()
                        .ok_or_else(|| anyhow!("Invalid path `{}`", path.path().display()))?,
                );
                paths_str.push_str(":");
            }
            Ok(paths_str)
        })
}

pub fn cheat_paths(config: &Config) -> Result<String, Error> {
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
