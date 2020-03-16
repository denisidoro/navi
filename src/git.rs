pub fn meta(uri: &str) -> (String, String, String) {
    let actual_uri = if uri.contains("://") {
        uri.to_string()
    } else if uri.contains('@') {
        uri.replace(':', "/").replace("git@", "https://")
    } else {
        format!("https://github.com/{}", uri)
    };

    let parts: Vec<&str> = actual_uri.split('/').collect();
    let user = parts[parts.len() - 2];
    let repo = parts[parts.len() - 1].replace(".git", "");

    (actual_uri.clone(), user.to_string(), repo)
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
        assert_eq!(actual_uri, "https://github.com/denisidoro/navi.git".to_string());
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
