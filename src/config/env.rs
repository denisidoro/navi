use crate::env_var;
use crate::finder::FinderChoice;
use crate::prelude::*;

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
            config_yaml: env_var::get(env_var::CONFIG_YAML).ok(),
            config_path: env_var::get(env_var::CONFIG).ok(),
            path: env_var::get(env_var::PATH).ok(),
            finder: env_var::get(env_var::FINDER)
                .ok()
                .and_then(|x| FinderChoice::from_str(&x).ok()),
            fzf_overrides: env_var::get(env_var::FZF_OVERRIDES).ok(),
            fzf_overrides_var: env_var::get(env_var::FZF_OVERRIDES_VAR).ok(),
        }
    }
}
