use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn follow_symlink(pathbuf: PathBuf) -> PathBuf {
    let other = fs::read_link(pathbuf.clone());
    match other {
        Ok(o) => {
            let o_str = o.as_os_str().to_str().unwrap();
            if o_str.starts_with('.') {
                let parent_str = pathbuf.parent().unwrap().as_os_str().to_str().unwrap();
                let path_str = format!("{}/{}", parent_str, o_str);
                let p = PathBuf::from(path_str);
                follow_symlink(p)
            } else {
                follow_symlink(o)
            }
        }
        Err(_) => pathbuf,
    }
}

fn exe_pathbuf() -> PathBuf {
    let pathbuf = std::env::current_exe().unwrap();
    follow_symlink(pathbuf)
}

pub fn exe_string() -> String {
    exe_pathbuf().as_os_str().to_str().unwrap().to_string()
}

pub fn exe_path_string() -> String {
    exe_pathbuf()
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}
