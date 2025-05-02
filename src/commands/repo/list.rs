use crate::common::git;
use crate::config::CONFIG;
use crate::filesystem::{all_cheat_files, default_cheat_pathbuf};
use crate::libs::terminal::hyperlink;
use crate::prelude::*;

pub fn main() {
    let mut cheats_repos: Vec<String> = Vec::new();
    let cheats = CONFIG.path().unwrap_or_else(|| {
        // if we don't have a path, use the default value
        let mut _cheats = default_cheat_pathbuf().unwrap();
        _cheats.push("navi");
        _cheats.push("cheats");

        _cheats.to_string()
    });

    // We're checking each given paths possible
    for cheat_path in cheats.split(':') {
        // If the path doesn't exist, continue to the next one
        if !std::fs::exists(cheat_path).unwrap() {
            continue;
        }

        let curr_dir = std::fs::read_dir(cheat_path).unwrap();

        // We're checking subfolders -> they should contain at least one .cheat files
        for entry in curr_dir {
            let entry = entry.unwrap();

            if entry.file_type().unwrap().is_dir() {
                // If the directory doesn't have at least one cheat file -> ignore it and continue
                if all_cheat_files(&entry.path()).is_empty() {
                    continue;
                };

                // If the directory have at least one cheat file -> add it to the list
                // Note: for the list, we are registering the git remote name and not the
                //      folder name since we modify it internally.
                let git_path = format!("{}/{}", &entry.path().display(), ".git");

                if std::fs::exists(&git_path).unwrap() {
                    let remote_uri = git::get_remote(&entry.path().to_string()).unwrap();

                    cheats_repos.push(remote_uri);
                } else {
                    cheats_repos.push(entry.path().display().to_string());
                }
            }
        }
    }

    // Now that we have our list of cheatsheet repositories, we loop through them
    // Two behaviours:
    // We do have entries -> We show them
    // We do not have entries -> We put a message for the user to add one
    if cheats_repos.is_empty() {
        eprintln!("Uh Oh! It seems you haven't downloaded a cheatsheet repository yet.");
        eprintln!("What you can do: \n\n- `navi repo add` to add a cheatsheet repository\n- `navi repo browse` to browse recommended cheatsheet repositories");

        // We quit this function
        return;
    }

    // The list shouldn't be empty
    eprintln!("You have locally available the following cheatsheet repositories: \n");

    for cheat_repo in cheats_repos {
        let content = if cheat_repo.starts_with("https://") {
            // If the URL is using the HTTPS scheme, we use a hyperlink
            // instead of printing its raw value.

            hyperlink::new(&cheat_repo, &cheat_repo)
        } else {
            cheat_repo.to_owned()
        };

        eprintln!("- {}", content);
    }
}
