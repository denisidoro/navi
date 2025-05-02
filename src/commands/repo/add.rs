use crate::common::git;
use crate::filesystem::{all_git_files, all_cheat_files, remove_dir, default_cheat_pathbuf, create_dir, tmp_pathbuf};
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::FinderChoice;
use crate::prelude::*;
use std::fs;
use std::path;
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};


fn ask_if_should_import_all(finder: &FinderChoice) -> Result<bool> {
    let opts = FinderOpts {
        column: Some(1),
        header: Some("Do you want to import all files from this repo?".to_string()),
        ..Default::default()
    };

    let (response, _) = finder
        .call(opts, |stdin| {
            stdin
                .write_all(b"Yes\nNo")
                .context("Unable to writer alternatives")?;
            Ok(())
        })
        .context("Unable to get response")?;

    Ok(response.to_lowercase().starts_with('y'))
}

pub fn main(uri: String) -> Result<()> {
    let finder = CONFIG.finder();

    let should_import_all = ask_if_should_import_all(&finder).unwrap_or(false);
    let (actual_uri, user, repo) = git::meta(uri.as_str());

    
    // TODO: Using the default cheat pathbuf will send the downloaded cheatsheets
    // into the path without taking into account the user-defined paths.
    let cheat_pathbuf = default_cheat_pathbuf()?;
    let tmp_pathbuf = tmp_pathbuf()?;
    let tmp_path_str = &tmp_pathbuf.to_string();

    let _ = remove_dir(&tmp_pathbuf);
    create_dir(&tmp_pathbuf)?;

    eprintln!("Cloning {} into {}...\n", &actual_uri, &tmp_path_str);

    git::shallow_clone(actual_uri.as_str(), tmp_path_str)
        .with_context(|| format!("Failed to clone `{actual_uri}`"))?;

    let all_files = all_cheat_files(&tmp_pathbuf).join("\n");
    let git_files = all_git_files(&tmp_pathbuf).join("\n");

    let opts = FinderOpts {
        suggestion_type: SuggestionType::MultipleSelections,
        preview: Some(format!("cat '{tmp_path_str}/{{}}'")),
        header: Some("Select the cheatsheets you want to import with <TAB> then hit <Enter>\nUse Ctrl-R for (de)selecting all".to_string()),
        preview_window: Some("right:30%".to_string()),
        ..Default::default()
    };

    let files = if should_import_all {
        all_files
    } else {
        let (files, _) = finder
            .call(opts, |stdin| {
                stdin
                    .write_all(all_files.as_bytes())
                    .context("Unable to prompt cheats to import")?;
                Ok(())
            })
            .context("Failed to get cheatsheet files from finder")?;
        files
    };

    let to_folder = {
        let mut p = cheat_pathbuf;
        p.push(format!("{user}__{repo}"));
        p
    };

    for file in files.split('\n') {
        let from = {
            let mut p = tmp_pathbuf.clone();
            p.push(file);
            p
        };
        let filename = file
            .replace(&format!("{}{}", &tmp_path_str, path::MAIN_SEPARATOR), "")
            .replace(path::MAIN_SEPARATOR, "__");
        let to = {
            let mut p = to_folder.clone();
            p.push(filename);
            p
        };
        fs::create_dir_all(&to_folder).unwrap_or(());
        fs::copy(&from, &to)
            .with_context(|| format!("Failed to copy `{}` to `{}`", &from.to_string(), &to.to_string()))?;
    }

    // We are copying the .git folder along with the cheat files
    // For more details, see: (https://github.com/denisidoro/navi/issues/733)
    for file in git_files.split('\n') {
        let filename = format!("{}{}", &tmp_path_str, &file);
        let from = {
            let mut p = tmp_pathbuf.clone();
            p.push(&filename);
            p
        };
        let to = {
            let mut p = to_folder.clone();
            p.push(&file);
            p
        };

        let path_str = &PathBuf::clone(&to).to_string();
        let local_collection = &path_str.split(MAIN_SEPARATOR).collect::<Vec<&str>>();
        let local_to_folder = format!("{}{}", &to_folder.to_string(), local_collection[0..&local_collection.len()-1].join(MAIN_SEPARATOR_STR));

        fs::create_dir_all(&local_to_folder).unwrap_or(());
        fs::copy(&from, format!("{}{}", &to_folder.clone().to_str().unwrap(), &to.clone().to_str().unwrap()))
            .with_context(|| format!("Failed to copy `{}` to `{}`", &from.to_string(), &to.to_string()))?;
    }

    remove_dir(&tmp_pathbuf)?;

    eprintln!(
        "The following .cheat files were imported successfully:\n{}\n\nThey are now located at {}",
        files,
        to_folder.to_string()
    );

    Ok(())
}
