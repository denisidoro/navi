use crate::env_var;
use crate::filesystem::default_config_pathbuf;
use crate::finder::FinderChoice;
use crate::fs;
use crate::terminal::style::Color as TerminalColor;
use anyhow::Result;
use serde::{de, Deserialize};
use std::convert::TryFrom;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub struct EnvConfig {
    pub config_yaml: Option<String>,
    pub config_path: Option<String>,
    pub path: Option<String>,
    pub shell: Option<String>,
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
            shell: env_var::get(env_var::SHELL).ok(),
            finder: env_var::get(env_var::FINDER)
                .ok()
                .and_then(|x| FinderChoice::from_str(&x).ok()),
            fzf_overrides: env_var::get(env_var::FZF_OVERRIDES).ok(),
            fzf_overrides_var: env_var::get(env_var::FZF_OVERRIDES_VAR).ok(),
        }
    }
}
