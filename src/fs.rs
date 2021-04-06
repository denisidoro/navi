use anyhow::{Context, Error};
use core::fmt::Display;
use remove_dir_all::remove_dir_all;
use std::fmt::Debug;
use std::fs::{self, create_dir_all, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
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

pub fn read_lines<P>(filename: P) -> Result<impl Iterator<Item = Result<String, Error>>, Error>
where
    P: AsRef<Path> + Display + Copy,
{
    let file = File::open(filename).with_context(|| format!("Failed to open file {}", filename))?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.map_err(Error::from)))
}

pub fn pathbuf_to_string(pathbuf: &Path) -> Result<String, Error> {
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
                let p = pathbuf
                    .parent()
                    .ok_or_else(|| anyhow!("`{}` has no parent", pathbuf.display()))?;
                let mut p = PathBuf::from(p);
                p.push(o_str);
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
    pathbuf_to_string(&exe_pathbuf()?)
}

pub fn create_dir(path: &Path) -> Result<(), Error> {
    create_dir_all(path).with_context(|| {
        format!(
            "Failed to create directory `{}`",
            pathbuf_to_string(path).expect("Unable to parse {path}")
        )
    })
}

pub fn remove_dir(path: &Path) -> Result<(), Error> {
    remove_dir_all(path).with_context(|| {
        format!(
            "Failed to remove directory `{}`",
            pathbuf_to_string(path).expect("Unable to parse {path}")
        )
    })
}
