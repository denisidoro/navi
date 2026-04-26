use crate::env_var;
use crate::finder::FinderChoice;
use std::str::FromStr;

#[derive(Debug)]
pub struct EnvConfig {
    pub config_yaml: Option<String>,
    pub config_path: Option<String>,
    pub path: Option<String>,
    pub finder: Option<FinderChoice>,
    pub fzf_overrides: Option<String>,
    pub fzf_overrides_var: Option<String>,
}

impl EnvConfig {
    pub fn new() -> Self {
        Self {
            config_yaml: env_var::parse(env_var::CONFIG_YAML),
            config_path: env_var::parse(env_var::CONFIG),
            path: env_var::parse(env_var::PATH),
            finder: env_var::parse(env_var::FINDER).and_then(|x: String| FinderChoice::from_str(&x).ok()),
            fzf_overrides: env_var::parse(env_var::FZF_OVERRIDES),
            fzf_overrides_var: env_var::parse(env_var::FZF_OVERRIDES_VAR),
        }
    }
}
