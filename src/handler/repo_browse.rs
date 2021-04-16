use crate::config::CONFIG;
use crate::filesystem;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::{Finder};
use crate::fs::pathbuf_to_string;
use crate::git;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io::Write;

pub fn main() -> Result<String> {
    let finder = CONFIG.finder;

    let repo_pathbuf = {
        let mut p = filesystem::tmp_pathbuf()?;
        p.push("featured");
        p
    };

    let repo_path_str = pathbuf_to_string(&repo_pathbuf)?;

    let _ = filesystem::remove_dir(&repo_pathbuf);
    filesystem::create_dir(&repo_pathbuf)?;

    let (repo_url, _, _) = git::meta("denisidoro/cheats");
    git::shallow_clone(repo_url.as_str(), &repo_path_str)
        .with_context(|| format!("Failed to clone `{}`", repo_url))?;

    let feature_repos_file = {
        let mut p = repo_pathbuf.clone();
        p.push("featured_repos.txt");
        p
    };

    let repos = fs::read_to_string(&feature_repos_file).context("Unable to fetch featured repositories")?;

    let opts = FinderOpts {
        column: Some(1),
        suggestion_type: SuggestionType::SingleSelection,
        ..Default::default()
    };

    let (repo, _, _) = finder
        .call(opts, |stdin, _| {
            stdin
                .write_all(repos.as_bytes())
                .context("Unable to prompt featured repositories")?;
            Ok(None)
        })
        .context("Failed to get repo URL from finder")?;

    filesystem::remove_dir(&repo_pathbuf)?;

    Ok(repo)
}
