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

            git::pull(&repo_path)?;
        } else {
            eprintln!("I don't find {} locally, are you sure you downloaded it?", &name);
        }

        return Ok(());
    }

    // We haven't been given a name -> We synchronize every cheatsheet repository we've found
    for cheat_repo in cheats_repo_paths {
        // TODO: Sanitize the cheatsheet folder of any file that is not a cheat file
        // Ref: https://github.com/denisidoro/navi/issues/733

        // Note for later, git considers the files as deleted
        // maybe we should sanitize the repo before doing the actual pull and then
        // reintegrate the logic found in repo/add.rs ?
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

        // TODO: A note to be remembered, the filter needs to be adjusted to take everything we do not want
        //  want to keep (i.e. anything that is not a git file nor a cheat file)

        let files_to_discard = WalkDir::new(&cheat_repo)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| {
                let path_str = e.path().to_str().unwrap_or("").to_string();

                if cheat_files.contains(&path_str) {
                    eprintln!("CHEAT: {}", &path_str);

                    return "".to_string()
                }

                return e.path().to_str().unwrap_or("").to_string()
            })
            .filter(|e| e.ends_with(".cheat") || e.ends_with(".cheat.md"))
            .collect::<Vec<String>>();

        for file_to_discard in files_to_discard.iter() {
            eprintln!("=> file to discard: {:?}", file_to_discard);
        }
    }


    Ok(())
}
