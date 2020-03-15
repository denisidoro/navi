use crate::cheat::SuggestionType;
use crate::filesystem;
use crate::fzf;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn add(uri: String) -> Result<(), Box<dyn Error>> {
    let mut parts = uri.split('/');
    let user = parts.next().unwrap();
    let repo = parts.next().unwrap();
    let cheat_path_str = filesystem::pathbuf_to_string(filesystem::cheat_pathbuf().unwrap());
    let repo_folder_str = format!("{}-master", repo);
    let tmp_path_str = format!("{}/tmp", cheat_path_str);

    // TODO: stop shell'ing out
    let cmd = format!(
        r#"
      url="https://github.com/{user}/{repo}/archive/master.tar.gz";
      mkdir -p "{tmp_path}"; 
      cd "{tmp_path}"; 
      (wget -c "$url" -O - || curl -L "$url") | tar -xz || exit 42;
      cd "{repo_folder}"; 
      find . -name "*.cheat"
    "#,
        tmp_path = tmp_path_str,
        repo_folder = repo_folder_str,
        user = user,
        repo = repo
    );

    let child = Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Unable to download or extract repository");

    let all_files = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();

    let opts = fzf::Opts {
        suggestion_type: SuggestionType::MultipleSelections,
        preview: false,
        header: Some(
            "Select the cheatsheets you want to import with <TAB> then hit <Enter>".to_string(),
        ),
        ..Default::default()
    };

    let (files, _) = fzf::call(opts, |stdin| {
        stdin
            .write_all(all_files.as_bytes())
            .expect("Unable to prompt cheats to import");
        None
    });

    for f in files.split('\n') {
        let from = format!("{}/{}/{}", tmp_path_str, repo_folder_str, f).replace("./", "");
        let to_folder = format!("{}/{}__{}", cheat_path_str, user, repo).replace("./", "");
        let filename = f.replace("./", "").replace("/", "__");
        let to = format!("{}/{}", to_folder, filename);
        fs::create_dir_all(to_folder).unwrap_or(());
        fs::copy(from, to)?;
    }

    fs::remove_dir_all(tmp_path_str).unwrap_or(());

    Ok(())
}
