use crate::common::shell::ShellSpawnError;
use crate::filesystem::remove_dir;
use crate::prelude::*;
use git2::{BranchType, Repository, ResetType};
use std::fmt::{Display, Error, Formatter};
use std::process::Command;

pub struct CheatRepositoryRecord {
    path: String,
    uri: String,
}

impl CheatRepositoryRecord {
    pub fn new(path: String, uri: String) -> Self {
        Self { path, uri }
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

    pub fn uri(&self) -> String {
        self.uri.clone()
    }
}

impl Display for CheatRepositoryRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.uri, self.path)
    }
}

pub fn shallow_clone(remote_uri: &str, target: &str, branch: &Option<String>, overwrite: bool) -> Result<()> {
    let target_path = PathBuf::from(target);

    // Check if the folder exists and the target is not empty
    if target_path.exists() && !target.is_empty() {
        if !overwrite {
            println!("{} already exists, skipping", target);

            return Ok(());
        }

        // We remove the folder before cloning it back
        remove_dir(target_path.as_path()).expect(
            format!(
                "Failed to clean {} before cloning the {} cheatsheet repository.",
                target, remote_uri
            )
            .as_str(),
        );
    }

    // Prepare fetch options.
    let mut builder = git2::build::RepoBuilder::new();
    if branch.is_some() {
        builder.branch(branch.as_ref().unwrap());
    }

    println!("Cloning {} to {}", remote_uri, target);
    builder.clone(remote_uri, target.as_ref()).expect(format!("Failed to clone {}", remote_uri).as_str());

    Ok(())
}

/// Gets a URI from a string and returns a set of three Strings representing:
/// - the expected URI (prefixed by `https://github.com/` if no signs of a remote are detected)
/// - the expected user behind the repository
/// - the name of the repository
pub fn meta_from_uri(uri: &str) -> (String, String, String) {
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
///
/// # Examples
///
/// ```
/// get_remote_uri()
/// ```
pub fn get_remote_uri(repository: Repository) -> String {
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

/// Takes a repository instance and updates it to its remote equivalent with a hard reset.
///
/// 3 steps function:
/// - Fetch latest tree state
/// - Grabs the latest commit ID
/// - hard reset to the latest commit
pub fn reset_to_remote_state(repository: &Repository) -> Result<()> {
    repository.find_remote("origin")?.fetch(&["main"], None, None)?;

    // Get the latest commit ID from the remote branch (origin/main)
    let latest_commit_id = repository.find_branch("origin/main", BranchType::Remote)?.into_reference().peel_to_commit()?;

    // Hard reset onto this commit
    repository.reset(latest_commit_id.as_object(), ResetType::Hard, None)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_github_https() {
        let (actual_uri, user, repo) = meta_from_uri("https://github.com/denisidoro/navi");
        assert_eq!(actual_uri, "https://github.com/denisidoro/navi".to_string());
        assert_eq!(user, "denisidoro".to_string());
        assert_eq!(repo, "navi".to_string());
    }

    #[test]
    fn test_meta_github_ssh() {
        let (actual_uri, user, repo) = meta_from_uri("git@github.com:denisidoro/navi.git");
        assert_eq!(actual_uri, "git@github.com:denisidoro/navi.git".to_string());
        assert_eq!(user, "denisidoro".to_string());
        assert_eq!(repo, "navi".to_string());
    }

    #[test]
    fn test_meta_gitlab_https() {
        let (actual_uri, user, repo) = meta_from_uri("https://gitlab.com/user/repo.git");
        assert_eq!(actual_uri, "https://gitlab.com/user/repo.git".to_string());
        assert_eq!(user, "user".to_string());
        assert_eq!(repo, "repo".to_string());
    }
}
