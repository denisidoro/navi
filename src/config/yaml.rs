use super::env::EnvConfig;
use crate::common::fs;
use crate::filesystem::default_config_pathbuf;
use crate::finder::FinderChoice;
use crate::prelude::*;
use crossterm::style::Color as TerminalColor;
use serde::de;

#[derive(Deserialize, Debug)]
pub struct Color(#[serde(deserialize_with = "color_deserialize")] TerminalColor);

impl Color {
    pub fn get(&self) -> TerminalColor {
        self.0
    }
}

fn color_deserialize<'de, D>(deserializer: D) -> Result<TerminalColor, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    TerminalColor::try_from(s.as_str())
        .map_err(|_| de::Error::custom(format!("Failed to deserialize color: {s}")))
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ColorWidth {
    pub color: Color,
    pub width_percentage: u16,
    pub min_width: u16,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Style {
    pub tag: ColorWidth,
    pub comment: ColorWidth,
    pub snippet: ColorWidth,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Finder {
    #[serde(deserialize_with = "finder_deserialize")]
    pub command: FinderChoice,
    pub overrides: Option<String>,
    pub overrides_var: Option<String>,
}

fn finder_deserialize<'de, D>(deserializer: D) -> Result<FinderChoice, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    FinderChoice::from_str(s.to_lowercase().as_str())
        .map_err(|_| de::Error::custom(format!("Failed to deserialize finder: {s}")))
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Cheats {
    pub path: Option<String>,
    pub paths: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Search {
    pub tags: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Shell {
    pub command: String,
    pub finder_command: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
#[derive(Default)]
pub struct Client {
    pub tealdeer: bool,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct YamlConfig {
    pub style: Style,
    pub finder: Finder,
    pub cheats: Cheats,
    pub search: Search,
    pub shell: Shell,
    pub client: Client,
}

impl YamlConfig {
    fn from_str(text: &str) -> Result<Self> {
        serde_yaml::from_str(text).map_err(|e| e.into())
    }

    fn from_path(path: &Path) -> Result<Self> {
        let file = fs::open(path)?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| e.into())
    }

    pub fn get(env: &EnvConfig) -> Result<Self> {
        if let Some(yaml) = env.config_yaml.as_ref() {
            return Self::from_str(yaml);
        }
        if let Some(path_str) = env.config_path.as_ref() {
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
            color: Color(TerminalColor::Blue),
            width_percentage: 26,
            min_width: 20,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            tag: ColorWidth {
                color: Color(TerminalColor::Cyan),
                width_percentage: 26,
                min_width: 20,
            },
            comment: ColorWidth {
                color: Color(TerminalColor::Blue),
                width_percentage: 42,
                min_width: 45,
            },
            snippet: Default::default(),
        }
    }
}

impl Default for Finder {
    fn default() -> Self {
        Self {
            command: FinderChoice::Fzf,
            overrides: None,
            overrides_var: None,
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            command: "bash".to_string(),
            finder_command: None,
        }
    }
}
