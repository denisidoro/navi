use super::env::EnvConfig;
use crate::common::fs;
use crate::filesystem::default_config_pathbuf;
use crate::finder::FinderChoice;
use crate::prelude::*;
use crossterm::style::Color as TerminalColor;
use serde::de;
use crate::common::fs::ToStringExt;

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
    pub delimiter_var: Option<String>,
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

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct YamlConfig {
    pub style: Style,
    pub finder: Finder,
    pub cheats: Cheats,
    pub search: Search,
    pub shell: Shell,
    pub client: Client,
    pub source: String, // <= The source of the current configuration
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

    pub fn get(env: &EnvConfig) -> Result<YamlConfig> {
        if let Some(yaml) = env.config_yaml.as_ref() {
            // We're getting the configuration from the environment variable `NAVI_CONFIG_YAML`
            let mut cfg = Self::from_str(yaml)?;
            cfg.source = format!("NAVI_CONFIG_YAML={}", yaml);

            return Ok(cfg);
        }
        if let Some(path_str) = env.config_path.as_ref() {
            // We're getting the configuration from a file given in the environment variable 'NAVI_CONFIG'

            let p = PathBuf::from(path_str);
            let mut cfg = YamlConfig::from_path(&p)?;
            cfg.source = format!("NAVI_CONFIG_YAML={:?}", p).to_string();

            return Ok(cfg);
        }
        if let Ok(p) = default_config_pathbuf() {
            // We're getting the configuration from the default path

            if p.exists() {
                let mut cfg = YamlConfig::from_path(&p)?;
                cfg.source = p.to_string();

                return Ok(cfg);
            }
        }

        // AS no configuration has been found, we set the YAML configuration
        // to be its default (built-in) value.
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
            delimiter_var: None,
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


impl Default for YamlConfig {
    fn default() -> Self {
        Self {
            style: Default::default(),
            finder: Default::default(),
            cheats: Default::default(),
            search: Default::default(),
            shell: Default::default(),
            client: Default::default(),
            source: "built-in".to_string(),
        }
    }
}
