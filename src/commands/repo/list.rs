use crate::filesystem::local_cheatsheet_repositories;
use crate::libs::terminal::hyperlink;

pub fn main() {
    let (cheats_repos, cheats_paths) = local_cheatsheet_repositories();

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

    let mut i: usize = 0;
    for cheat_repo in cheats_repos {
        let content = if cheat_repo.starts_with("https://") {
            // If the URL is using the HTTPS scheme, we use a hyperlink
            // instead of printing its raw value.

            hyperlink::new(&cheat_repo, &cheat_repo)
        } else {
            hyperlink::new(&format!("file://{}", &cheats_paths[i]), &cheat_repo)
        };

        eprintln!("- {}", content);
        i += 1;
    }
}
