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
        if let Some((domain, route)) = uri.split_once('/') {
            if domain.contains(".") {
                format!("https://{domain}/{route}")
            } else {
                // Users can pass name starting wirh a slash
                let first_char = uri.chars().next();
                
                if first_char == Some('/') {
                    format!("https://github.com{uri}")
                }
                else {
                    format!("https://github.com/{uri}")
                }
            }
        } else {
            panic!("Invalid link")
        }
    };

    let uri_to_split = actual_uri.replace(':', "/");
    let parts: Vec<&str> = uri_to_split.split('/').collect();
    let user = parts[parts.len() - 2];
    let repo = parts[parts.len() - 1].replace(".git", "");

    (actual_uri, user.to_string(), repo)
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
    
    #[test]
    fn test_meta_github_repo_name() {
        let (actual_uri, user, repo) = meta("user/repo");
        assert_eq!(actual_uri, "https://github.com/user/repo".to_string());
        assert_eq!(user, "user".to_string());
        assert_eq!(repo, "repo".to_string());
    }
    
    #[test]
    fn test_meta_github_repo_name_with_slash() {
        let (actual_uri, user, repo) = meta("/user/repo");
        assert_eq!(actual_uri, "https://github.com/user/repo".to_string());
        assert_eq!(user, "user".to_string());
        assert_eq!(repo, "repo".to_string());
    }
    
    #[test]
    fn test_meta_github_clean_link() {
        let (actual_uri, user, repo) = meta("github.com/user/repo");
        assert_eq!(actual_uri, "https://github.com/user/repo".to_string());
        assert_eq!(user, "user".to_string());
        assert_eq!(repo, "repo".to_string());
    }
    
    #[test]
    fn test_meta_random_git_provider_http() {
        let (actual_uri, user, repo) = meta("https://sr.ht/user/repo");
        assert_eq!(actual_uri, "https://sr.ht/user/repo".to_string());
        assert_eq!(user, "user".to_string());
        assert_eq!(repo, "repo".to_string());
    }
}
