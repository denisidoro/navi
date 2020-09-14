use crate::common::git;
use crate::filesystem;
use crate::finder::{Finder, FinderChoice};
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use anyhow::Context;
use anyhow::Error;
use std::fs;
use std::io::Write;

pub fn browse(finder: &FinderChoice) -> Result<(), Error> {
    let repo_path_str = format!("{}/featured", filesystem::tmp_path_str()?);

    let _ = filesystem::remove_dir(&repo_path_str);
    filesystem::create_dir(&repo_path_str)?;

    let (repo_url, _, _) = git::meta("denisidoro/cheats");
    git::shallow_clone(repo_url.as_str(), &repo_path_str).with_context(|| format!("Failed to clone `{}`", repo_url))?;

    let repos = fs::read_to_string(format!("{}/featured_repos.txt", &repo_path_str)).context("Unable to fetch featured repositories")?;

    let opts = FinderOpts {
        column: Some(1),
        ..Default::default()
    };

    let (repo, _) = finder
        .call(opts, |stdin| {
            stdin.write_all(repos.as_bytes()).context("Unable to prompt featured repositories")?;
            Ok(None)
        })
        .context("Failed to get repo URL from finder")?;

    filesystem::remove_dir(&repo_path_str)?;

    add(repo, finder)
}

pub fn add(uri: String, finder: &FinderChoice) -> Result<(), Error> {
    let (actual_uri, user, repo) = git::meta(uri.as_str());

    let cheat_path_str = filesystem::pathbuf_to_string(filesystem::default_cheat_pathbuf()?)?;
    let tmp_path_str = filesystem::tmp_path_str()?;

    let _ = filesystem::remove_dir(&tmp_path_str);
    filesystem::create_dir(&tmp_path_str)?;

    eprintln!("Cloning {} into {}...\n", &actual_uri, &tmp_path_str);

    git::shallow_clone(actual_uri.as_str(), &tmp_path_str).with_context(|| format!("Failed to clone `{}`", actual_uri))?;

    let all_files = filesystem::all_cheat_files(&tmp_path_str)
        .join("\n");

    let opts = FinderOpts {
        suggestion_type: SuggestionType::MultipleSelections,
        preview: Some(format!("cat '{}/{{}}'", tmp_path_str)),
        header: Some("Select the cheatsheets you want to import with <TAB> then hit <Enter>".to_string()),
        preview_window: Some("right:30%".to_string()),
        ..Default::default()
    };

    let (files, _) = finder
        .call(opts, |stdin| {
            stdin.write_all(all_files.as_bytes()).context("Unable to prompt cheats to import")?;
            Ok(None)
        })
        .context("Failed to get cheatsheet files from finder")?;

    for f in files.split('\n') {
        let from = format!("{}/{}", tmp_path_str, f).replace("./", "");
        let to_folder = format!("{}/{}__{}", cheat_path_str, user, repo).replace("./", "");
        let filename = f.replace("./", "").replace("/", "__");
        let to = format!("{}/{}", to_folder, filename);
        fs::create_dir_all(to_folder).unwrap_or(());
        fs::copy(&from, &to).with_context(|| format!("Failed to copy `{}` to `{}`", from, to))?;
    }

    filesystem::remove_dir(&tmp_path_str)?;

    eprintln!(
        "The following .cheat files were imported successfully:\n{}\n\nThey are now located at {}\n\nPlease run navi again to check the results.",
        files, cheat_path_str
    );

    Ok(())
}
