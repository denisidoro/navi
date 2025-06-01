use crate::common::git;
use crate::filesystem::{
    all_cheat_files, all_git_files, create_dir, default_cheat_pathbuf, remove_dir, tmp_pathbuf,
};
use crate::finder::questions::finder_yes_no_question;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::FinderChoice;
use crate::prelude::*;
use std::fs;
use std::path;
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use tracing_subscriber::fmt::format;

fn ask_if_should_import_all(finder: &FinderChoice) -> Result<bool> {
    let opts = FinderOpts {
        column: Some(1),
        header: Some("Do you want to import all files from this repo?".to_string()),
        ..Default::default()
    };

    finder_yes_no_question(finder, opts)
}
fn ask_folder_present_question(finder: &FinderChoice) -> Result<bool> {
    let opts = FinderOpts {
        column: Some(1),
        header: Some(
            "It seems this cheatsheet repository has been previously added, do you still want to continue?"
                .to_string(),
        ),
        ..Default::default()
    };

    finder_yes_no_question(finder, opts)
}

pub fn main(uri: String, yes_flag: bool, branch: &Option<String>) -> Result<()> {
    let finder = CONFIG.finder();

    // If the user has set the yes flag, we don't ask a confirmation
    let should_import_all = if yes_flag {
        true
    } else {
        ask_if_should_import_all(&finder).unwrap_or(false)
    };

    let (actual_uri, user, repo) = git::meta(uri.as_str());

    // TODO: Using the default cheat pathbuf will send the downloaded cheatsheets
    //  into the path without taking into account the user-defined paths.
    let cheat_pathbuf = default_cheat_pathbuf()?;
    let tmp_pathbuf = tmp_pathbuf()?;
    let tmp_path_str = &tmp_pathbuf.to_string();
    let to_folder = {
        let mut p = cheat_pathbuf;
        p.push(format!("{user}__{repo}"));
        p
    };

    // Before anything else, we check to see if the folder exists
    // if it exists -> ask confirmation if we continue
    if fs::exists(&to_folder)? {
        // When the yes_flag has been raised => follow through and removes the existing directory
        // When the yes_flag has not been raised => ask for confirmation
        if yes_flag || ask_folder_present_question(&finder).unwrap_or(false) {
            fs::remove_dir_all(&to_folder)?;
        } else {
            return Ok(());
        }
    }

    let _ = remove_dir(&tmp_pathbuf);
    create_dir(&tmp_pathbuf)?;

    eprintln!("Cloning {} into {}...\n", &actual_uri, &tmp_path_str);

    git::shallow_clone(actual_uri.as_str(), tmp_path_str, &branch)
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
            .with_context(|| format!("Failed to copy cheat file `{}` to `{}`", &from.to_string(), &to.to_string()))?;
    }

    // We are copying the .git folder along with the cheat files
    // For more details, see: (https://github.com/denisidoro/navi/issues/733)
    for file in git_files.split('\n') {
        let file_path = format!("{}{}", &tmp_path_str, &file);
        let from = {
            let mut p = tmp_pathbuf.clone();
            p.push(&file_path);
            p
        };
        
        eprintln!("{file_path}");


        let path_str = format!("{}{}{}", to_folder.to_string(), path::MAIN_SEPARATOR, &file_path);
        let local_collection = &path_str.split(MAIN_SEPARATOR).collect::<Vec<&str>>();
        let collection_str = if cfg!(windows) {
            eprintln!("{:?}", local_collection);

            local_collection[1..&local_collection.len() - 1].join(MAIN_SEPARATOR_STR)
        } else {
            local_collection[0..&local_collection.len() - 1].join(MAIN_SEPARATOR_STR)
        };

        // This should be able to fix an issue with the clone on windows where both
        // to_folder and collection_str are equal
        let local_to_folder = if &to_folder.to_string() != &collection_str {
            format!(
                "{}{}",
                &to_folder.to_string(),
                &collection_str
            )
        } else {
            to_folder.to_string()
        };

        // This should be able to fix an issue with the clone on windows where both
        // to_folder and collection_str are equal
        let complete_local_path = if &to_folder.to_string() != &collection_str {
            format!(
                "{}{}",
                &to_folder.to_string(),
                &collection_str
            )
        } else {
            to_folder.to_string()
        };


        eprintln!("=> (&to_folder.to_string() == &collection_str) = {}", &to_folder.to_string() == &collection_str);
        eprintln!("=> To_folder: {}", &to_folder.to_string());
        eprintln!("=> Collection: {}", &collection_str);
        eprintln!("=> local_to_folder: {}", &local_to_folder);
        eprintln!("=> complete_local_path: {}", &complete_local_path);

        debug!("=> {}", &complete_local_path);

        fs::create_dir_all(&local_to_folder).unwrap_or(());
        fs::copy(&from, &complete_local_path).with_context(|| {
            format!(
                "Failed to copy git file `{}` to `{}`",
                &from.to_string(),
                &complete_local_path
            )
        })?;
    }

    remove_dir(&tmp_pathbuf)?;

    eprintln!(
        "The following .cheat files were imported successfully:\n{}\n\nThey are now located at {}",
        files,
        to_folder.to_string()
    );

    Ok(())
}
