use crate::handler;
use crate::shell::{self, ShellSpawnError};
use crate::structures::config;
use crate::url;
use anyhow::Error;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Func {
    UrlOpen,
    Welcome,
    WidgetLastCommand,
    MapExpand,
}

pub fn main(func: &Func, args: Vec<String>) -> Result<(), Error> {
    match func {
        Func::UrlOpen => url::open(args),
        Func::Welcome => handler::handle_config(config::config_from_iter(
            "navi --path /tmp/navi/irrelevant".split(' ').collect(),
        )),
        Func::WidgetLastCommand => widget_last_command(),
        Func::MapExpand => map_expand(),
    }
}

fn map_expand() -> Result<(), Error> {
    let cmd = r#"sed -e 's/^.*$/"&"/' | tr '\n' ' '"#;
    shell::command()
        .arg("-c")
        .arg(cmd)
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;
    Ok(())
}

fn widget_last_command() -> Result<(), Error> {
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
