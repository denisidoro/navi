use crate::common::git;
use crate::filesystem::local_cheatsheet_repositories;
use crate::prelude::*;

pub fn main(name: Option<String>) -> Result<()> {
    let (cheats_repo_uris, cheats_repo_paths) = local_cheatsheet_repositories();

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
        eprintln!("Pulling the latest version of {}", cheat_repo);

        git::pull(&cheat_repo)?;
    }

    // TODO: Sanitize the cheatsheet folder of any file that is not a cheat file
    // Ref: https://github.com/denisidoro/navi/issues/733

    // Note for later, git considers the files as deleted
    // maybe we should sanitize the repo before doing the actual pull and then
    // reintegrate the logic found in repo/add.rs ?

    Ok(())
}
