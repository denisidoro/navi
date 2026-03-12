use std::fs;
use std::path::MAIN_SEPARATOR;
use crate::filesystem::{all_cheat_files, all_git_files, local_cheatsheet_repositories};
use crate::prelude::*;
use git2::Repository;

use crate::common::git::reset_to_remote_state;
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

        let repository = cheat_repos.iter()
            .filter(|repository| repository.uri().eq(given_uri))
            .next().unwrap();

        let repository_object = Repository::open(repository.path())?;

        synchronize(&repository_object).expect("Unable to sync repository");

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
    let repo_path = cheat_repo.path().parent().unwrap();

    // 1 - Before doing an actual reset, we remove the current cheatsheets
    let mut cheat_files = all_cheat_files(repo_path);

    for cheat_file in cheat_files {

        fs::remove_file(Path::new(cheat_file.as_str()))
            .with_context(|| format!("Unable to remove {}", cheat_file))?;
    }

    // 2 - We reset the repository to its remote equivalent
    reset_to_remote_state(cheat_repo)?;

    // 3 - We reshuffle the files
    cheat_files = all_cheat_files(repo_path);

    for cheat_file in cheat_files {
        let source = Path::new(cheat_file.as_str());

        let filename = cheat_file.replace(&format!("{}", &repo_path.to_str().unwrap()), "")
                .replace(MAIN_SEPARATOR, "__");

        let destination = repo_path.join(filename);
        println!("[Repo::sync](DEBUG) - {} => {:?}", cheat_file, destination);

        fs::copy(source, &destination).with_context(|| format!("Unable to copy {} to {}", cheat_file, destination.display()))?;

        fs::remove_file(source).with_context(|| format!("Unable to remove {}", cheat_file))?;
    }

    Ok(())
}
