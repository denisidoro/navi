use crate::display;
use crate::parser;
use crate::structures::cheat::Suggestion;
use crate::structures::{config::Config, error::command::BashSpawnError};
use anyhow::Context;
use anyhow::Error;
use std::env;
use std::process::{Command, Stdio};

pub fn main(config: Config) -> Result<(), Error> {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .spawn()
        .context("Unable to create child")?;
    let stdin = child.stdin.as_mut().context("Unable to get stdin")?;
    let mut writer = display::alfred::Writer::new();

    display::alfred::print_items_start(None);

    parser::read_all(&config, stdin, &mut writer)
        .context("Failed to parse variables intended for finder")?;

    // make sure everything was printed to stdout before attempting to close the items vector
    let _ = child.wait_with_output().context("Failed to wait for fzf")?;

    display::alfred::print_items_end();
    Ok(())
}

fn prompt_with_suggestions(suggestion: &Suggestion) -> Result<String, Error> {
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

pub fn suggestions(config: Config, dry_run: bool) -> Result<(), Error> {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .context("Unable to create child")?;
    let stdin = child.stdin.as_mut().context("Unable to get stdin")?;
    let mut writer = display::alfred::Writer::new();

    let variables = parser::read_all(&config, stdin, &mut writer)
        .context("Failed to parse variables intended for finder")?;

    let tags = env::var("tags").context(r#"The env var "tags" isn't set"#)?;
    let snippet = env::var("snippet").context(r#"The env var "snippet" isn't set"#)?;

    let capture = display::VAR_REGEX.captures_iter(&snippet).next();
    let bracketed_varname = &(capture.expect("Invalid capture"))[0];
    let varname = &bracketed_varname[1..bracketed_varname.len() - 1];
    let command = variables.get(&tags, &varname);

    if dry_run {
        if command.is_none() {
            println!("{}", varname);
        }
        return Ok(());
    }

    display::alfred::print_items_start(Some(varname));

    let command = command.context("Invalid command")?;
    let lines = prompt_with_suggestions(command).context("Invalid lines")?;

    writer.reset();

    for line in lines.split('\n') {
        writer.write_suggestion(&snippet, &varname, &line);
    }

    display::alfred::print_items_end();

    Ok(())
}

pub fn transform() -> Result<(), Error> {
    let snippet = env::var("snippet").context(r#"The env var "snippet" isn't set"#)?;
    let varname = env::var("varname").context(r#"The env var "varname" isn't set"#)?;
    let value = if let Ok(v) = env::var(&varname) {
        v
    } else {
        env::var("free").context("The env var for varname isn't set")?
    };

    let bracketed_varname = format!("<{}>", varname);
    let interpolated_snippet = snippet.replace(&bracketed_varname, &value);
    println!("{}", interpolated_snippet);

    Ok(())
}
