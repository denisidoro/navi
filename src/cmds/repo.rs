use std::error::Error;
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;

use crate::fzf;
use crate::filesystem;

pub fn main() -> Result<(), Box<dyn Error>> {
    let user = "denisidoro";
    let repo = "navi";
    let cheat_path = filesystem::cheat_pathbuf();
    let cheat_path_str = filesystem::pathbuf_to_string(cheat_path.unwrap());
    let repo_folder_str = format!("{}-master", repo);
    let tmp_path_str = format!("{}/tmp", cheat_path_str);

    let cmd = format!("url=\"https://github.com/{user}/{repo}/archive/master.tar.gz\" && mkdir -p \"{tmp_path}\"; cd \"{tmp_path}\"; (wget -c \"$url\" -O - || curl -L \"$url\") | tar -xz && cd \"{repo_folder}\"; find . -name \"*.cheat\"", 
    tmp_path = tmp_path_str,
    repo_folder = repo_folder_str,
    user = user, 
    repo = repo);

    let child = Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let all_files = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();

        let opts = fzf::Opts {
            multi: true,
            preview: false,
            header: Some("Select the cheatsheets you want to import with <TAB> then hit <Enter>".to_string()),
        ..Default::default()
    };

    let (files, _) = fzf::call(opts, |stdin| {
        stdin.write_all(all_files.as_bytes()).unwrap();
        None
    });

    println!("{}", files);

    for f in files.split('\n') {
        let from = format!("{}/{}/{}", tmp_path_str, repo_folder_str, f).replace("./", "");
        let to_folder = format!("{}/{}/{}", cheat_path_str, user, repo).replace("./", "");
        let to = format!("{}/{}", to_folder, f).replace("./", "");
        println!("{} -> {}", from, to);
        fs::create_dir_all(to_folder).unwrap_or(());
        fs::copy(from, to)?;
    }

    Ok(())
}
