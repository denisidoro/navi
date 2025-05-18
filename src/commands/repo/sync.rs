use std::fs;
use walkdir::WalkDir;
use crate::common::git;
use crate::filesystem::{all_cheat_files, all_git_files, local_cheatsheet_repositories};
use crate::prelude::*;

use crate::commands::repo::HELP_NO_REPOSITORIES_FOUND;

pub fn main(name: Option<String>) -> Result<()> {
    let (cheats_repo_uris, cheats_repo_paths) = local_cheatsheet_repositories();

    if cheats_repo_paths.is_empty() {
        eprintln!("{}", HELP_NO_REPOSITORIES_FOUND);

        return Ok(());
    }

    if name.clone().is_some() {
        let name = name.clone().unwrap();

        // We have been given a repository uri to check
        if cheats_repo_uris.contains(&name) {
            let folder_index = cheats_repo_uris.iter().position(|r| r == &name).unwrap();
            let repo_path = cheats_repo_paths[folder_index].clone();

            synchronize(repo_path)?;
        } else {
            eprintln!("I don't find {} locally, are you sure you downloaded it?", &name);
        }

        return Ok(());
    }

    // We haven't been given a name -> We synchronize every cheatsheet repository we've found
    for cheat_repo in cheats_repo_paths {
        synchronize(cheat_repo)?;
    }


    Ok(())
}

fn synchronize(cheat_repo: String) -> Result<()> {
    let cheat_path = Path::new(&cheat_repo);
    eprintln!("Checking repo {:?}", &cheat_path);

    // We retrieve all existing cheat files
    let cheat_files = all_cheat_files(&cheat_path);
    let git_files = all_git_files(&cheat_path);

    // We delete them since they are now out of tree
    for file in cheat_files.iter() {
        fs::remove_file(&file)?;
    }

    // Now that the folder has been cleaned, we need to fetch the latest HEAD available.
    git::fetch_origin(&cheat_repo)?;
    git::pull(&cheat_repo)?;

    // Now that we've checkout the repository to the latest commit,
    // we might have a surplus of "illegal" files (i.e. files that should not be present in a cheatsheet repository).
    //
    // They need to be removed and the cheat files renamed.

    let files_to_discard = WalkDir::new(&cheat_repo)
        .follow_links(true)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| {
            if e.path().is_dir() {
                return "".to_string();
            }

            let path_str = e.path().to_str().unwrap_or("").to_string();

            if cheat_files.contains(&path_str) {

                return "".to_string()
            }

            // We substract the path of the cheatsheet root folder to let us get
            // a matching path for the git_file's condition.
            let cheat_str = cheat_path.display().to_string();
            let cheat_str = cheat_str.as_str();
            let cheat_str = &path_str.clone().replace(cheat_str, "");


            if git_files.contains(&cheat_str) {

                return "".to_string();
            }

            return e.path().display().to_string();
        })
        .filter(|e| e != "")
        .collect::<Vec<String>>();

    // We're discarding all the files that we don't want
    for file_to_discard in files_to_discard.iter() {
        debug!("=> file to discard: {:?}", file_to_discard);

        fs::remove_file(&file_to_discard)?;
    }

    // TODO: We should flatten the folder just like the behaviour of `navi repo add`

    Ok(())
}
