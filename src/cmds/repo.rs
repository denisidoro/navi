use crate::filesystem;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::{Finder, FinderChoice};
use crate::fs::pathbuf_to_string;
use crate::git;
use anyhow::Context;
use anyhow::Error;
use std::fs;
use std::io::Write;

pub fn browse(finder: &FinderChoice) -> Result<(), Error> {
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

    let (repo, _) = finder
        .call(opts, &mut Vec::new(), |stdin, _| {
            stdin
                .write_all(repos.as_bytes())
                .context("Unable to prompt featured repositories")?;
            Ok(None)
        })
        .context("Failed to get repo URL from finder")?;

    filesystem::remove_dir(&repo_pathbuf)?;

    add(repo, finder)
}

pub fn ask_if_should_import_all(finder: &FinderChoice) -> Result<bool, Error> {
    let opts = FinderOpts {
        column: Some(1),
        header: Some("Do you want to import all files from this repo?".to_string()),
        ..Default::default()
    };

    let (response, _) = finder
        .call(opts, &mut Vec::new(), |stdin, _| {
            stdin
                .write_all(b"Yes\nNo")
                .context("Unable to writer alternatives")?;
            Ok(None)
        })
        .context("Unable to get response")?;

    if response.to_lowercase().starts_with('y') {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn add(uri: String, finder: &FinderChoice) -> Result<(), Error> {
    let should_import_all = ask_if_should_import_all(finder).unwrap_or(false);
    let (actual_uri, user, repo) = git::meta(uri.as_str());

    let cheat_pathbuf = filesystem::default_cheat_pathbuf()?;
    let tmp_pathbuf = filesystem::tmp_pathbuf()?;
    let tmp_path_str = pathbuf_to_string(&tmp_pathbuf)?;

    let _ = filesystem::remove_dir(&tmp_pathbuf);
    filesystem::create_dir(&tmp_pathbuf)?;

    eprintln!("Cloning {} into {}...\n", &actual_uri, &tmp_path_str);

    git::shallow_clone(actual_uri.as_str(), &tmp_path_str)
        .with_context(|| format!("Failed to clone `{}`", actual_uri))?;

    let all_files = filesystem::all_cheat_files(&tmp_pathbuf).join("\n");

    let opts = FinderOpts {
        suggestion_type: SuggestionType::MultipleSelections,
        preview: Some(format!("cat '{}/{{}}'", tmp_path_str)),
        header: Some("Select the cheatsheets you want to import with <TAB> then hit <Enter>\nUse Ctrl-R for (de)selecting all".to_string()),
        preview_window: Some("right:30%".to_string()),
        ..Default::default()
    };

    let files = if should_import_all {
        all_files
    } else {
        let (files, _) = finder
            .call(opts, &mut Vec::new(), |stdin, _| {
                stdin
                    .write_all(all_files.as_bytes())
                    .context("Unable to prompt cheats to import")?;
                Ok(None)
            })
            .context("Failed to get cheatsheet files from finder")?;
        files
    };

    let to_folder = {
        let mut p = cheat_pathbuf.clone();
        p.push(format!("{}__{}", user, repo));
        p
    };

    for file in files.split('\n') {
        let from = {
            let mut p = tmp_pathbuf.clone();
            p.push(file);
            p
        };
        let filename = file.replace("/", "__");
        let to = {
            let mut p = to_folder.clone();
            p.push(filename);
            p
        };
        fs::create_dir_all(&to_folder).unwrap_or(());
        fs::copy(&from, &to).with_context(|| {
            format!(
                "Failed to copy `{}` to `{}`",
                pathbuf_to_string(&from).expect("unable to parse {from}"),
                pathbuf_to_string(&to).expect("unable to parse {to}")
            )
        })?;
    }

    filesystem::remove_dir(&tmp_pathbuf)?;

    eprintln!(
        "The following .cheat files were imported successfully:\n{}\n\nThey are now located at {}/{}",
        files,
        pathbuf_to_string(&cheat_pathbuf)?,
        pathbuf_to_string(&to_folder)?
    );

    Ok(())
}
