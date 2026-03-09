use crate::common::git;
use crate::filesystem::{all_cheat_files, all_git_files, default_cheat_pathbuf, running_cheats_path, tmp_pathbuf, JOIN_SEPARATOR};
use crate::finder::questions::finder_yes_no_question;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::FinderChoice;
use crate::prelude::*;
use std::{fs};
use std::path::{MAIN_SEPARATOR};

fn ask_if_should_import_all(finder: &FinderChoice) -> Result<bool> {
    let opts = FinderOpts {
        column: Some(1),
        header: Some("Do you want to import all files from this repo?".to_string()),
        ..Default::default()
    };

    finder_yes_no_question(finder, opts)
}

pub fn main(uri: String, yes_flag: bool, branch: &Option<String>) -> Result<()> {
    /////////////////////////////////////////////////////////////////////////////
    // 0 - Setup/Fetch/Compute any necessary values/variables to use afterwards
    /////////////////////////////////////////////////////////////////////////////

    let finder = CONFIG.finder();
    // We're getting the values from the URI
    let (_, user, repo_name) = git::meta_from_uri(&*uri);
    let local_cheatsheet_repository_name = format!("{user}__{repo_name}");

    // We're grabbing the first path that actually exists on the filesystem
    // where we can store the cheatsheet repository
    let cheat_paths_string = running_cheats_path();
    let cheat_paths = cheat_paths_string.split(JOIN_SEPARATOR);
    let mut cheat_pathbuf: PathBuf = PathBuf::from(cheat_paths.clone().last().unwrap());

    for cheat_path in cheat_paths {
        let local_path = PathBuf::from(cheat_path);
        if ! local_path.exists() {
            continue;
        }

        cheat_pathbuf = local_path;
        break
    };

    // We need to be sure we've had at least one path to store the cheatsheet repository
    if cheat_pathbuf.eq(&PathBuf::from("")) {
        eprintln!("Unable to find a path from the registered cheats paths, we fall back to the default one.");

        cheat_pathbuf = default_cheat_pathbuf().expect("Unable to get a default cheat path!")
    }

    // We add the interpolated cheatsheet repository name at the end of the path
    cheat_pathbuf.push(&local_cheatsheet_repository_name);

    /////////////////////////////////////////////////////////////////////////////
    // 1 - Create / Clean the temporary directory and clone the repository there
    /////////////////////////////////////////////////////////////////////////////

    let tmp_base_pathbuf = tmp_pathbuf()?;
    let tmp_repository_pathbuf = {
        let mut p = tmp_base_pathbuf;
        p.push(&local_cheatsheet_repository_name);
        p
    };
    let tmp_repository_path_str = tmp_repository_pathbuf.to_str().unwrap();

    git::shallow_clone(&*uri, tmp_repository_path_str, branch, true).expect(format!("Failed to clone {uri} into {}", tmp_repository_path_str).as_str());

    // At this step, we're already registering the files of the repository
    let mut cheat_files = all_cheat_files(tmp_repository_pathbuf.as_ref());
    let git_files = all_git_files(&tmp_repository_pathbuf.as_ref());

    /////////////////////////////////////////////////////////////////////////////
    // 2 - Secure / Validate user input and preferences
    // (First and last user interaction in the workflow)
    /////////////////////////////////////////////////////////////////////////////

    // The yes flag makes us have an unattended installation of the repository
    let should_import_all = if yes_flag {
        true
    } else {
        ask_if_should_import_all(&finder).unwrap_or(false)
    };

    let finder_opts = FinderOpts {
        suggestion_type: SuggestionType::MultipleSelections,
        preview: Some(format!("cat '{}/{{}}'", tmp_repository_path_str)),
        header: Some("Select the cheatsheets you want to import with <TAB> then hit <Enter>\nUse Ctrl-R for (de)selecting all".to_string()),
        preview_window: Some("right:30%".to_string()),
        ..Default::default()
    };

    // Now, we have to ask the user that doesn't want to import all cheatsheets
    // which one he wants, we then update the actual list of files we want to import
    if !should_import_all {
        let (files, _) = finder
            .call(finder_opts, |stdin| {
                stdin
                    .write_all(cheat_files.join("\n").as_bytes())
                    .context("Unable to prompt cheats to import")?;
                Ok(())
            })
            .context("Failed to get cheat files from finder")?;

        // We reattribute
        cheat_files = files.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
    }

    /////////////////////////////////////////////////////////////////////////////
    // 3 - Resolve the final destination of the cheatsheet repository and move
    // the sanitized repository there
    /////////////////////////////////////////////////////////////////////////////

    let all_files = [cheat_files, git_files.clone()].concat();

    for current_file in all_files {
        let current_file_extension = Path::new(current_file.as_str()).extension();
        let current_file_extension_str = if current_file_extension.is_some() {
            current_file_extension.unwrap().to_str().unwrap().to_string()
        } else {
            "".to_string()
        };

        let tmp_cheat_file = {
            let mut p = tmp_repository_pathbuf.clone();
            p.push(&current_file);
            p
        };
        // Filename
        let filename = if current_file_extension_str.contains("cheat") {
            current_file
                .replace(&format!("{}{}", &tmp_repository_path_str, MAIN_SEPARATOR), "")
                .replace(MAIN_SEPARATOR, "__")
        } else {
            current_file
                .replace(&format!("{}{}", &tmp_repository_path_str, MAIN_SEPARATOR), "")
        };
        let final_file = {
            let mut p = cheat_pathbuf.clone();
            p.push(filename);
            p
        };

        println!("[Repo::add](DEBUG) - {}", final_file.display());

        let parent = final_file.parent().unwrap().to_str().unwrap();

        // We're now moving the files into their final folders
        fs::create_dir_all(parent).with_context(|| format!("Unable to create {}", final_file.display())).unwrap_or(());

        fs::copy(&tmp_cheat_file, &final_file)
            .with_context(|| format!(
                "Failed to copy `{}` to `{}`!",
                tmp_cheat_file.to_str().unwrap(),
                &final_file.to_str().unwrap())
            )?;
    }


    println!("Temp cheats folder: {:?}", &tmp_repository_path_str);
    println!("Final cheats folder: {:?}", cheat_pathbuf.to_string());

    Ok(())
}
