use crate::cheat::SuggestionType;
use crate::filesystem;
use crate::fzf;
use git2::Repository;
use std::error::Error;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;

fn create_dir(path: &str) {
    fs::create_dir_all(path).unwrap_or(());
}

fn remove_dir(path: &str) {
    fs::remove_dir_all(path).unwrap_or(());
}

pub fn add(uri: String) -> Result<(), Box<dyn Error>> {
    let actual_uri = if uri.contains("://") {
        uri
    } else {
        format!("https://github.com/{}", uri)
    };

    let parts: Vec<&str> = actual_uri.split('/').collect();
    let user = parts[parts.len() - 2];
    let repo = parts[parts.len() - 1].replace(".git", "");

    let cheat_path_str = filesystem::pathbuf_to_string(filesystem::cheat_pathbuf().unwrap());
    let tmp_path_str = format!("{}/tmp", cheat_path_str);
    let tmp_path_str_with_trailing_slash = format!("{}/", &tmp_path_str);

    remove_dir(&tmp_path_str);
    create_dir(&tmp_path_str);

    eprintln!("Cloning {} into {}...\n", &actual_uri, &tmp_path_str);

    match Repository::clone(actual_uri.as_str(), &tmp_path_str) {
        Ok(r) => r,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let all_files = WalkDir::new(&tmp_path_str)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .filter(|e| e.ends_with(".cheat"))
        .map(|e| e.replace(&tmp_path_str_with_trailing_slash, ""))
        .collect::<Vec<String>>()
        .join("\n");

    let overrides = "--preview-window right:30%".to_string();
    let opts = fzf::Opts {
        suggestion_type: SuggestionType::MultipleSelections,
        preview: Some(format!("cat {}/{{}}", tmp_path_str_with_trailing_slash)),
        header: Some(
            "Select the cheatsheets you want to import with <TAB> then hit <Enter>".to_string(),
        ),
        overrides: Some(&overrides),
        ..Default::default()
    };

    let (files, _) = fzf::call(opts, |stdin| {
        stdin
            .write_all(all_files.as_bytes())
            .expect("Unable to prompt cheats to import");
        None
    });

    for f in files.split('\n') {
        let from = format!("{}/{}", tmp_path_str, f).replace("./", "");
        let to_folder = format!("{}/{}__{}", cheat_path_str, user, repo).replace("./", "");
        let filename = f.replace("./", "").replace("/", "__");
        let to = format!("{}/{}", to_folder, filename);
        fs::create_dir_all(to_folder).unwrap_or(());
        fs::copy(from, to)?;
    }

    remove_dir(&tmp_path_str);

    eprintln!("The following .cheat files were imported successfully:\n{}\n\nThey are now located at {}\n\nPlease run navi again to check the results.", files, cheat_path_str);

    Ok(())
}
