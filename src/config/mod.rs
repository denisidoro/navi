mod cli;
mod env;
mod yaml;

use crate::finder::FinderChoice;

use crate::terminal::style::Color;
pub use cli::*;

use env::EnvConfig;
use yaml::YamlConfig;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
pub struct Config {
    yaml: YamlConfig,
    clap: ClapConfig,
    env: EnvConfig,
}

impl Config {
    pub fn new() -> Self {
        let env = EnvConfig::new();
        let yaml = YamlConfig::get(&env).unwrap_or_else(|e| {
            eprintln!("Error parsing config file: {}", e);
            eprintln!("Fallbacking to default one...");
            eprintln!();
            YamlConfig::default()
        });
        let clap = ClapConfig::new();
        Self { yaml, clap, env }
    }

    pub fn best_match(&self) -> bool {
        self.clap.best_match
    }

    pub fn cmd(&self) -> Option<&Command> {
        self.clap.cmd.as_ref()
    }

    pub fn source(&self) -> Source {
        if let Some(query) = self.clap.tldr.clone() {
            Source::Tldr(query)
        } else if let Some(query) = self.clap.cheatsh.clone() {
            Source::Cheats(query)
        } else {
            Source::Filesystem(self.path(), self.tag_rules())
        }
    }

    pub fn path(&self) -> Option<String> {
        self.clap
            .path
            .clone()
            .or_else(|| self.env.path.clone())
            .or_else(|| {
                let p = self.yaml.cheats.paths.clone();
                if p.is_empty() {
                    None
                } else {
                    Some(p.join(":"))
                }
            })
            .or_else(|| self.yaml.cheats.path.clone())
    }

    pub fn finder(&self) -> FinderChoice {
        self.clap
            .finder
            .or(self.env.finder)
            .unwrap_or(self.yaml.finder.command)
    }

    pub fn fzf_overrides(&self) -> Option<String> {
        self.clap
            .fzf_overrides
            .clone()
            .or_else(|| self.env.fzf_overrides.clone())
            .or_else(|| self.yaml.finder.overrides.clone())
    }

    pub fn fzf_overrides_var(&self) -> Option<String> {
        self.clap
            .fzf_overrides_var
            .clone()
            .or_else(|| self.env.fzf_overrides_var.clone())
            .or_else(|| self.yaml.finder.overrides_var.clone())
    }

    pub fn shell(&self) -> String {
        self.yaml.shell.command.clone()
    }

    pub fn finder_shell(&self) -> String {
        if let Some(c) = self.yaml.shell.finder_command.clone() {
            c
        } else {
            self.yaml.shell.command.clone()
        }
    }

    pub fn tag_rules(&self) -> Option<String> {
        self.clap
            .tag_rules
            .clone()
            .or_else(|| self.yaml.search.tags.clone())
    }

    pub fn tag_color(&self) -> Color {
        self.yaml.style.tag.color.get()
    }

    pub fn comment_color(&self) -> Color {
        self.yaml.style.comment.color.get()
    }

    pub fn snippet_color(&self) -> Color {
        self.yaml.style.snippet.color.get()
    }

    pub fn tag_width_percentage(&self) -> u16 {
        self.yaml.style.tag.width_percentage
    }

    pub fn comment_width_percentage(&self) -> u16 {
        self.yaml.style.comment.width_percentage
    }

    pub fn tag_min_width(&self) -> u16 {
        self.yaml.style.tag.min_width
    }

    pub fn comment_min_width(&self) -> u16 {
        self.yaml.style.comment.min_width
    }

    pub fn action(&self) -> Action {
        if self.clap.print {
            Action::Print
        } else {
            Action::Execute
        }
    }

    pub fn get_query(&self) -> Option<String> {
        let q = self.clap.query.clone();
        if q.is_some() {
            return q;
        }
        if self.best_match() {
            match self.source() {
                Source::Tldr(q) => Some(q),
                Source::Cheats(q) => Some(q),
                _ => Some(String::from("")),
            }
        } else {
            None
        }
    }
}
