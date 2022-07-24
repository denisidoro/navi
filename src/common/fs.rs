use super::prelude::*;
use directories_next::BaseDirs;
use std::ffi::OsStr;

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

pub fn config_dir(project_name: &str) -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().context("unable to get base dirs")?;

    let mut pathbuf = PathBuf::from(base_dirs.config_dir());
    pathbuf.push(project_name);
    Ok(pathbuf)
}
