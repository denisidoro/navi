use crate::filesystem::local_cheatsheet_repositories;
use crate::libs::terminal::hyperlink;
use anyhow::Context;

use crate::commands::repo::HELP_NO_REPOSITORIES_FOUND;

pub fn main() {
    let cheat_repos = local_cheatsheet_repositories()
        .context("Unable to retrieve local cheatsheet repositories.")
        .unwrap();

    // Now that we have our list of cheatsheet repositories, we loop through them
    // Two behaviours:
    // We do have entries -> We show them
    // We do not have entries -> We put a message for the user to add one
    if cheat_repos.is_empty() {
        println!("{}", HELP_NO_REPOSITORIES_FOUND);

        // We quit this function
        return;
    }

    // The list shouldn't be empty
    println!("The following cheatsheets are installed on your device: \n");

    for cheat_repo in cheat_repos {
        let cheat_path = cheat_repo.path();
        let cheat_uri = cheat_repo.uri();

        let content_link = if cheat_repo.is_remote() {
            // If the URL is using the HTTPS scheme, we use a hyperlink
            // instead of printing its raw value.

            hyperlink::new(&cheat_uri, &cheat_uri)
        } else {
            hyperlink::new(&format!("file://{}", &cheat_uri), &cheat_uri)
        };

        println!("- {} ({})\n", content_link, cheat_path);
    }
}
