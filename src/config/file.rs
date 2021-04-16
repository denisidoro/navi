use crate::env_var;
use crate::finder::FinderChoice;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
#[serde(default)]
pub struct ColorWidth {
    pub color: String,
    pub width: u16,
    pub min_abs_width: u16,
}
#[derive(Deserialize)]
#[serde(default)]
pub struct Style {
    pub tag: ColorWidth,
    pub comment: ColorWidth,
    pub snippet: ColorWidth,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Finder {
    pub command: FinderChoice,
    pub overrides: Option<String>,
    pub overrides_var: Option<String>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Cheats {
    pub path: Option<String>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Search {
    pub tags: Option<String>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Shell {
    pub command: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Yaml {
    pub style: Style,
    pub finder: Finder,
    pub cheats: Cheats,
    pub search: Search,
    pub shell: Shell,
}

impl Yaml {
    pub fn parse(text: &str) -> Self {
        serde_yaml::from_str::<Yaml>(&text).expect("invalid yaml")
    }
}

impl Default for ColorWidth {
    fn default() -> Self {
        Self {
            color: "cyan".to_string(),
            width: 26,
            min_abs_width: 20,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            tag: ColorWidth {
                color: "cyan".to_string(),
                width: 26,
                min_abs_width: 20,
            },
            comment: ColorWidth {
                color: "blue".to_string(),
                width: 42,
                min_abs_width: 45,
            },
            snippet: ColorWidth {
                color: "white".to_string(),
                width: 0,
                min_abs_width: 0,
            },
        }
    }
}

impl Default for Finder {
    fn default() -> Self {
        Self {
            command: env_var::get(env_var::FINDER)
                .ok()
                .and_then(|x| FinderChoice::from_str(&x).ok())
                .unwrap_or(FinderChoice::Fzf),
            overrides: env_var::get(env_var::FZF_OVERRIDES).ok(),
            overrides_var: env_var::get(env_var::FZF_OVERRIDES_VAR).ok(),
        }
    }
}

impl Default for Cheats {
    fn default() -> Self {
        Self {
            path: env_var::get(env_var::PATH).ok(),
        }
    }
}

impl Default for Search {
    fn default() -> Self {
        Self { tags: None }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            command: env_var::get(env_var::SHELL)
                .ok()
                .unwrap_or_else(|| "bash".to_string()),
        }
    }
}
