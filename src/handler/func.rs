use crate::config::{Config, CONFIG};
use crate::filesystem;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::Finder;
use crate::handler::core;
use crate::shell::{self, ShellSpawnError};
use crate::structures::cheat::VariableMap;
use crate::url;
use crate::welcome;
use anyhow::Context;
use anyhow::Result;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Func {
    UrlOpen,
    Welcome,
    WidgetLastCommand,
    MapExpand,
}

pub fn main(func: &Func, args: Vec<String>) -> Result<()> {
    match func {
        Func::UrlOpen => url::open(args),
        Func::Welcome => welcome(),
        Func::WidgetLastCommand => widget_last_command(),
        Func::MapExpand => map_expand(),
    }
}

fn welcome() -> Result<()> {
    let config = &CONFIG;
    let opts = core::gen_core_finder_opts(&config)?;
    let _ = config
        .finder
        .call(opts, |stdin, _| {
            welcome::populate_cheatsheet(stdin);
            Ok(Some(VariableMap::new()))
        })
        .context("Failed getting selection and variables from finder")?;
    Ok(())
}

fn map_expand() -> Result<()> {
    let cmd = r#"sed -e 's/^.*$/"&"/' | tr '\n' ' '"#;
    shell::command()
        .arg("-c")
        .arg(cmd)
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;
    Ok(())
}

fn widget_last_command() -> Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    let replacements = vec![("|", "ඛ"), ("||", "ග"), ("&&", "ඝ")];

    let parts = shellwords::split(&text).unwrap_or_else(|_| text.split('|').map(|s| s.to_string()).collect());

    for p in parts {
        for (pattern, escaped) in replacements.clone() {
            if p.contains(pattern) && p != pattern {
                let replacement = p.replace(pattern, escaped);
                text = text.replace(&p, &replacement);
            }
        }
    }

    let mut extracted = text.clone();
    for (pattern, _) in replacements.clone() {
        let mut new_parts = text.rsplit(pattern);
        if let Some(extracted_attempt) = new_parts.next() {
            if extracted_attempt.len() <= extracted.len() {
                extracted = extracted_attempt.to_string();
            }
        }
    }

    for (pattern, escaped) in replacements.clone() {
        text = text.replace(&escaped, &pattern);
        extracted = extracted.replace(&escaped, &pattern);
    }

    println!("{}", extracted.trim_start());

    Ok(())
}
