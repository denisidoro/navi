use std::str::FromStr;

use crate::env_var;
use crate::finder::FinderChoice;
use crate::terminal::style::Color;
pub struct ColorWidth {
    pub color: Color,
    pub width: u16,
    pub min_abs_width: u16,
}
pub struct Style {
    pub tag: ColorWidth,
    pub comment: ColorWidth,
    pub snippet: ColorWidth,
}

pub struct Finder {
    pub command: FinderChoice,
    pub overrides: Option<String>,
    pub overrides_var: Option<String>,
}

pub struct Cheats {
    pub path: Option<String>,
}

pub struct Search {
    pub tags: Option<String>,
}

pub struct Shell {
    pub command: String,
}
pub struct Yaml {
    pub style: Style,
    pub finder: Finder,
    pub cheats: Cheats,
    pub search: Search,
    pub shell: Shell,
}

impl Yaml {
    pub fn new() -> Self {
        Self {
            style: Style {
                tag: ColorWidth {
                    color: Color::Cyan,
                    width: 26,
                    min_abs_width: 20,
                },
                comment: ColorWidth {
                    color: Color::Blue,
                    width: 42,
                    min_abs_width: 45,
                },
                snippet: ColorWidth {
                    color: Color::White,
                    width: 0,
                    min_abs_width: 0,
                },
            },
            finder: Finder {
                command: env_var::get(env_var::FINDER)
                    .ok()
                    .and_then(|x| FinderChoice::from_str(&x).ok())
                    .unwrap_or(FinderChoice::Fzf),
                overrides: env_var::get(env_var::FZF_OVERRIDES).ok(),
                overrides_var: env_var::get(env_var::FZF_OVERRIDES_VAR).ok(),
            },
            cheats: Cheats {
                path: env_var::get(env_var::PATH).ok(),
            },
            search: Search { tags: None },
            shell: Shell {
                command: env_var::get(env_var::SHELL)
                    .ok()
                    .unwrap_or_else(|| "bash".to_string()),
            },
        }
    }
}
