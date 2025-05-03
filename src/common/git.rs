use crate::common::shell::ShellSpawnError;
use crate::prelude::*;
use std::process::Command;

pub fn shallow_clone(uri: &str, target: &str) -> Result<()> {
    Command::new("git")
        .args(["clone", uri, target, "--depth", "1"])
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
pub fn get_remote(uri: &str) -> Result<String> {
    // We consider the repository having only one remote
    // In case of multiple occurrences, we return the first one and discard the others

    let git_path = format!("{}/.git/", &uri);
    let mut remotes_uri: Vec<String> = Vec::new();

    if std::fs::exists(&git_path)? {
        // If the path exists, retrieve the remotes
        let remotes = Command::new("git")
            .current_dir(&git_path)
            .args(["remote"])
            .output()
            .context("Unable to git remote")?;

        // This is the name of the remote, not its URI
        let current_remote = String::from_utf8_lossy(&remotes.stdout).trim().to_string();

        let remote_uri = Command::new("git")
            .current_dir(&git_path)
            .args(["config", format!("remote.{}.url", current_remote).as_str()])
            .output()
            .context(format!(
                "Unable to git config remote <remote>.url for {}",
                &current_remote
            ))?;

        // This is the URI of the remote
        let current_remote_uri = String::from_utf8_lossy(&remote_uri.stdout).trim().to_string();

        remotes_uri.push(current_remote_uri);
    }

    Ok(remotes_uri[0].clone())
}

/// Pulls the latest version of a git repository
pub fn pull(uri: &str) -> Result<()> {
    Command::new("git")
        .current_dir(uri)
        .args(["pull", "origin"])
        .spawn()?
        .wait()
        .expect("Unable to git pull");
    Ok(())
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
