use crate::env_var;
use crate::filesystem::default_config_pathbuf;
use crate::finder::FinderChoice;
use crate::fs;
use crate::terminal::style;
use anyhow::Result;
use serde::{de, Deserialize};

use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Color(#[serde(deserialize_with = "color_deserialize")] style::Color);

impl Color {
    pub fn from_str(color: &str) -> Self {
        Self(style::Color::from_str(color).unwrap_or(style::Color::White))
    }

    pub fn get(&self) -> style::Color {
        self.0
    }
}

fn color_deserialize<'de, D>(deserializer: D) -> Result<style::Color, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    style::Color::from_str(&s).map_err(|_| de::Error::custom(format!("Failed to deserialize color: {}", s)))
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ColorWidth {
    pub color: Color,
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
pub struct YamlConfig {
    pub style: Style,
    pub finder: Finder,
    pub cheats: Cheats,
    pub search: Search,
    pub shell: Shell,
}

impl YamlConfig {
    fn from_str(text: &str) -> Result<Self> {
        serde_yaml::from_str(&text).map_err(|_| anyhow!("TODO"))
    }

    fn from_path(path: &Path) -> Result<Self> {
        let file = fs::open(path)?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|_| anyhow!("TODO"))
    }

    pub fn get() -> Result<Self> {
        if let Ok(yaml) = env_var::get(env_var::CONFIG_YAML) {
            return Self::from_str(&yaml);
        }
        if let Ok(path_str) = env_var::get(env_var::CONFIG) {
            let p = PathBuf::from(path_str);
            return YamlConfig::from_path(&p);
        }
        if let Ok(p) = default_config_pathbuf() {
            if p.exists() {
                return YamlConfig::from_path(&p);
            }
        }
        Ok(YamlConfig::default())
    }
}

impl Default for ColorWidth {
    fn default() -> Self {
        Self {
            color: Color::from_str("white"),
            width: 26,
            min_abs_width: 20,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            tag: ColorWidth {
                color: Color::from_str("cyan"),
                width: 26,
                min_abs_width: 20,
            },
            comment: ColorWidth {
                color: Color::from_str("blue"),
                width: 42,
                min_abs_width: 45,
            },
            snippet: Default::default(),
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
