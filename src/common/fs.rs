use crate::prelude::*;
use remove_dir_all::remove_dir_all;
use std::ffi::OsStr;
use std::fs::{self, create_dir_all, File};
use std::io;
use thiserror::Error;
use std::env::current_exe;

pub trait ToStringExt {
    fn to_string(&self) -> String;
}

impl ToStringExt for Path {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

impl ToStringExt for OsStr {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

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

pub fn open<P: AsRef<Path>>(filename: P) -> Result<File> {
    File::open(filename.as_ref()).with_context(|| {
        let x = filename.as_ref().to_string();
        format!("Failed to open file {}", &x)
    })
}

pub fn read_lines<P: AsRef<Path>>(filename: P) -> Result<impl Iterator<Item = Result<String>>> {
    let file = open(filename.as_ref())?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.map_err(Error::from)))
}

pub fn pathbuf_to_string<P: AsRef<Path>>(pathbuf: P) -> Result<String> {
    Ok(pathbuf.as_ref()
        .as_os_str()
        .to_str()
        .ok_or_else(|| InvalidPath(pathbuf.as_ref().to_path_buf()))
        .map(str::to_string)?)
}

fn follow_symlink(pathbuf: PathBuf) -> Result<PathBuf> {
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

fn exe_pathbuf() -> Result<PathBuf> {
    let pathbuf = current_exe().context("Unable to acquire executable's path")?;

    #[cfg(target_family = "windows")]
    let pathbuf = dunce::canonicalize(pathbuf)?;

    debug!(current_exe = ?pathbuf);
    follow_symlink(pathbuf)
}

fn exe_abs_string() -> Result<String> {
    pathbuf_to_string(&exe_pathbuf()?)
}

pub fn exe_string() -> String {
    exe_abs_string().unwrap_or_else(|_| "navi".to_string())
}

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    create_dir_all(path.as_ref()).with_context(|| {
        format!(
            "Failed to create directory `{}`",
            pathbuf_to_string(path).expect("Unable to parse {path}")
        )
    })
}

pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    remove_dir_all(path.as_ref()).with_context(|| {
        format!(
            "Failed to remove directory `{}`",
            pathbuf_to_string(path).expect("Unable to parse {path}")
        )
    })
}
