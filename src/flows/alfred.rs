use crate::parser;
use crate::structures::cheat::Suggestion;
use crate::structures::{error::command::BashSpawnError, option::Config};
use anyhow::Context;
use anyhow::Error;
use crate::display;
use std::env;
use std::process::{Command, Stdio};

pub fn main(config: Config) -> Result<(), Error> {
    let mut child = Command::new("cat").stdin(Stdio::piped()).spawn().unwrap();
    let stdin = child.stdin.as_mut().unwrap();
    display::alfred::print_items_start(None);
    parser::read_all(&config, stdin).context("Failed to parse variables intended for finder")?;
    let _ = child.wait_with_output().context("Failed to wait for fzf")?;
    display::alfred::print_items_end();
    Ok(())
}

fn prompt_with_suggestions(
    _variable_name: &str,
    _config: &Config,
    suggestion: &Suggestion,
) -> Result<String, Error> {
    let (suggestion_command, _suggestion_opts) = suggestion;

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

    let tags = env::var("tags").unwrap();
    let _comment = env::var("comment").unwrap();
    let snippet = env::var("snippet").unwrap();

    let varname = display::VAR_REGEX.captures_iter(&snippet).next();

    if let Some(varname) = varname {
        let varname = &varname[0];
        let varname = &varname[1..varname.len() - 1];

        display::alfred::print_items_start(Some(varname));

        let lines = variables
            .get(&tags, &varname)
            .ok_or_else(|| anyhow!("No suggestions"))
            .and_then(|suggestion| {
                Ok(prompt_with_suggestions(&varname, &config, suggestion).unwrap())
            })?;

let mut writer = display::alfred::new_writer();

        for line in lines.split('\n') {

writer.write_suggestion(&snippet, &varname, &line);
        }
    } else {
        display::alfred::print_items_start(None);
    }

    display::alfred::print_items_end();

    Ok(())
}

pub fn transform(_config: Config) -> Result<(), Error> {
    Ok(())
}
