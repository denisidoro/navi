use crate::clipboard;
use crate::display;
use crate::filesystem;
use crate::finder::Finder;
use crate::handler;
use crate::parser;
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use crate::structures::option;
use crate::structures::{error::command::BashSpawnError, option::Config};
use anyhow::Context;
use anyhow::Error;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

lazy_static! {
    pub static ref VAR_REGEX: Regex = Regex::new(r"<(\w[\w\d\-_]*)>").expect("Invalid regex");
}

pub fn main(config: Config) -> Result<(), Error> {
    let mut child = Command::new("cat").stdin(Stdio::piped()).spawn().unwrap();
    let stdin = child.stdin.as_mut().unwrap();

    println!(r#"{{"items": ["#);

    parser::read_all(&config, stdin).context("Failed to parse variables intended for finder")?;

    let _ = child.wait_with_output().context("Failed to wait for fzf")?;

    println!(r#"]}}"#);

    Ok(())
}

fn prompt_with_suggestions(
    variable_name: &str,
    config: &Config,
    suggestion: &Suggestion,
) -> Result<String, Error> {
    let (suggestion_command, suggestion_opts) = suggestion;

    let child = Command::new("bash")
        .stdout(Stdio::piped())
        .arg("-c")
        .arg(&suggestion_command)
        .spawn()
        .map_err(|e| BashSpawnError::new(suggestion_command, e))?;

    let suggestions = String::from_utf8(
        child
            .wait_with_output()
            .context("Failed to wait and collect output from bash")?
            .stdout,
    )
    .context("Suggestions are invalid utf8")?;

    Ok(suggestions)
}

pub fn suggestions(config: Config) -> Result<(), Error> {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    let stdin = child.stdin.as_mut().unwrap();

    let variables = parser::read_all(&config, stdin)
        .context("Failed to parse variables intended for finder")?;

    let variable_name = env::var("varname").unwrap();
    let tags = env::var("tags").unwrap();
    let comment = env::var("comment").unwrap();
    let snippet = env::var("snippet").unwrap();

    let varname = VAR_REGEX.captures_iter(&snippet).next();

    if varname.is_some() {
        let varname = &varname.unwrap()[0];

        variables
            .get(&tags, &variable_name)
            .ok_or_else(|| anyhow!("No suggestions"))
            .and_then(|suggestion| {
                let out = prompt_with_suggestions(&variable_name, &config, suggestion).unwrap();

                println!("{}", out);
                Ok(())
            })?;

        /*println!(
            r#"{{"variables": {{"varname": {varname}, "items": ["#,
            varname = varname
        );*/

        /*println!(r#"
                    {{
              "type": "file",
              "title": "lorem",
              "subtitle": "uber, url :: navi fn url::open https://ubunny.uberinternal.com/ubunny?q=eng+<query>",
              "variables": {{
                "{varname}": "lorem"
              }},
              "autocomplete": "Desktop",
              "icon": {{
                "type": "fileicon",
                "path": "~/Desktop"
              }}
            }}"#,
        varname = varname);*/

        //println!(r#"]}}"#);
    }

    Ok(())
}

pub fn transform(config: Config) -> Result<(), Error> {
    Ok(())
}
