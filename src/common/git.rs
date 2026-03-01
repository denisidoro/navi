use crate::common::shell::ShellSpawnError;
use crate::prelude::*;
use git2::Repository;
use std::process::Command;

pub struct CheatRepositoryRecord {
    path: String,
    uri: String,
    name: String,
}

impl CheatRepositoryRecord {
    pub fn new(path: String, uri: String, name: String) -> Self {
        Self { name, path, uri }
    }

    /// Returns if the URI of the repository seems to be remote or not
    pub fn is_remote(&self) -> bool {
        if self.uri.contains("://") || self.uri.contains('@') {
            true
        } else {
            false
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn uri(&self) -> String {
        self.uri.clone()
    }
}

pub fn shallow_clone(uri: &str, target: &str, branch: &Option<String>) -> Result<()> {
    // If we target a specific ref, we add the parameter inside the arguments to call
    // git with.

    let git_clone_args: Vec<&str> = if branch.is_some() {
        Vec::from([
            "clone",
            uri,
            target,
            "--depth",
            "1",
            "--branch",
            branch.as_ref().unwrap().as_str(),
        ])
    } else {
        Vec::from(["clone", uri, target, "--depth", "1"])
    };

    debug!("{}", format!("Launching 'git {git_clone_args:?}'"));

    Command::new("git")
        .args(git_clone_args)
        .spawn()
        .map_err(|e| ShellSpawnError::new("git clone", e))?
        .wait()
        .context("Unable to git clone")?;

    Ok(())
}

pub fn meta(uri: &str) -> (String, String, String) {
    let actual_uri = if uri.contains("://") || uri.contains('@') {
        uri.to_string()
    } else {
        format!("https://github.com/{uri}")
    };

    let uri_to_split = actual_uri.replace(':', "/");
    let parts: Vec<&str> = uri_to_split.split('/').collect();
    let user = parts[parts.len() - 2];
    let repo = parts[parts.len() - 1].replace(".git", "");

    (actual_uri, user.to_string(), repo)
}

/// Retrieves the remote URI of a git repository
/// Works best with a repository containing only one remote.
pub fn get_remote(repository: Repository) -> String {
    let remotes = repository.remotes().unwrap();
    let mut returned_remotes: Vec<String> = Vec::new();

    // Retrieve the remote URI
    remotes.iter().for_each(|remote| {
        let remote = repository.find_remote(remote.unwrap()).unwrap();
        let remote_url = remote.url().unwrap();
        returned_remotes.push(String::from(remote_url));
        println!("Remote: {:#}", remote_url);
    });

    returned_remotes.get(0).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_github_https() {
        let (actual_uri, user, repo) = meta("https://github.com/denisidoro/navi");
        assert_eq!(actual_uri, "https://github.com/denisidoro/navi".to_string());
        assert_eq!(user, "denisidoro".to_string());
        assert_eq!(repo, "navi".to_string());
    }

    #[test]
    fn test_meta_github_ssh() {
        let (actual_uri, user, repo) = meta("git@github.com:denisidoro/navi.git");
        assert_eq!(actual_uri, "git@github.com:denisidoro/navi.git".to_string());
        assert_eq!(user, "denisidoro".to_string());
        assert_eq!(repo, "navi".to_string());
    }

    #[test]
    fn test_meta_gitlab_https() {
        let (actual_uri, user, repo) = meta("https://gitlab.com/user/repo.git");
        assert_eq!(actual_uri, "https://gitlab.com/user/repo.git".to_string());
        assert_eq!(user, "user".to_string());
        assert_eq!(repo, "repo".to_string());
    }
}
