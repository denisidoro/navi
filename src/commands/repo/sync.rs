use crate::common::git;
use crate::filesystem::{all_cheat_files, all_git_files, local_cheatsheet_repositories};
use crate::prelude::*;
use git2::Repository;
use std::{fs, path};
use walkdir::WalkDir;

use crate::commands::repo::{HELP_NO_GIVEN_REPOSITORIES_FOUND, HELP_NO_REPOSITORIES_FOUND};

pub fn main(uri: Option<String>) -> Result<()> {
    let cheat_repos =
        local_cheatsheet_repositories().context("Unable to retrieve local cheatsheet repositories.")?;

    if cheat_repos.is_empty() {
        eprintln!("{}", HELP_NO_REPOSITORIES_FOUND);

        return Ok(());
    }

    // If we have a name of a repository, sync only this one and exit
    if uri.is_some() {
        let given_uri = uri.as_ref().unwrap();

        // let repository = cheat_repos.iter()
        //     .filter(|repository| repository == &given_uri)
        //     .next();

        println!("Repo: {given_uri:#}");

        // repository.map(|_| {
        //     let cheat_repo_index = cheats_repo_uris.iter().position(|repository| repository == given_uri);
        //     let given_repository = Repository::open(Path::new(cheats_repo_paths.get(cheat_repo_index.unwrap()).unwrap()));
        //     let repo_object = given_repository.as_ref().unwrap();
        //
        //     synchronize(repo_object).expect("Unable to sync repository");
        // }).expect(format!("{}", HELP_NO_GIVEN_REPOSITORIES_FOUND).as_str());

        return Ok(());
    }

    // We haven't been given a name -> We synchronize every cheatsheet repository we've found
    // for cheat_repo in cheat_repos {
    //     let given_repository = Repository::open(Path::new(cheat_repo.as_str()))?;
    //
    //     println!("Generic Repository: {}", given_repository.path().display());
    //     synchronize(&given_repository).expect("Unable to sync repository");
    // }

    Ok(())
}

fn synchronize(cheat_repo: &Repository) -> Result<()> {
    let cheat_path = cheat_repo.path();
    eprintln!("Checking repo {:?}", &cheat_path);

    // We retrieve all existing cheat files
    let cheat_files = all_cheat_files(&cheat_path);
    let git_files = all_git_files(cheat_path);
    let mut cheat_dirs: Vec<String> = Vec::new();

    Ok(())
}
