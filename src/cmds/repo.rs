use std::error::Error;
use std::process::{Command, Stdio};
use std::io::Write;

use crate::fzf;
use crate::filesystem;

pub fn main() -> Result<(), Box<dyn Error>> {
    let cheat_path = filesystem::cheat_pathbuf();
    let cheat_path_str = filesystem::pathbuf_to_string(cheat_path.unwrap());

    let cmd = format!("user=\"{user}\" && repo=\"{repo}\" && url=\"https://github.com/${{user}}/${{repo}}/archive/master.tar.gz\" && path=\"{cheat_path}/tmp\"; mkdir -p \"${{path}}\"; cd \"${{path}}\"; (wget -c \"$url\" -O - || curl -L \"$url\") | tar -xz && cd \"${{repo}}-master\"; find . -name \"*.cheat\"", cheat_path = cheat_path_str, user = "denisidoro", repo = "navi");

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

    Ok(())
}
