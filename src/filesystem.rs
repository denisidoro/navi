use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};
use std::fs::metadata;

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn pathbuf_to_string(pathbuf: PathBuf) -> String {
    pathbuf
    .as_os_str()
    .to_str()
    .unwrap()
    .to_string()
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

pub fn cheat_pathbuf() -> Option<PathBuf> {
    let exe_parent_str = exe_parent_string();

    let array = ["cheats", "../libexec/cheats", "../cheats", "../../cheats"];
    for elem in &array {
        let p = format!("{}/{}", exe_parent_str, elem);
        let meta = metadata(&p);
        if let Ok(m) = meta {
        if m.is_dir() {
            return Some(PathBuf::from(p));
        } 
    }
    }

    None
}

pub fn shell_pathbuf() -> PathBuf {
    let cheat_path_str = pathbuf_to_string(cheat_pathbuf().unwrap());
    PathBuf::from(format!("{}/../shell", cheat_path_str)) 
}

pub fn exe_string() -> String {
    pathbuf_to_string(exe_pathbuf())
}

pub fn exe_parent_string() -> String {
    exe_pathbuf()
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}
