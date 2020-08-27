use crate::common::clipboard;
use crate::common::shell::BashSpawnError;
use crate::display;
use crate::fetcher::Fetcher;
use crate::filesystem;
use crate::finder::Finder;
use crate::handler;
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::config;
use crate::structures::config::Config;
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use crate::structures::config::Command::Tldr;
use crate::welcome;
use anyhow::Context;
use anyhow::Error;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use crate::tldr;

pub enum Variant {
    Core,
    Filter(String),
    Query(String),
}

fn gen_core_finder_opts(variant: Variant, config: &Config) -> Result<FinderOpts, Error> {
    let mut opts = FinderOpts {
        preview: if config.no_preview {
            None
        } else {
            Some(format!("{} preview {{}}", filesystem::exe_string()?))
        },
        autoselect: !config.no_autoselect,
        overrides: config.fzf_overrides.clone(),
        suggestion_type: SuggestionType::SnippetSelection,
        ..Default::default()
    };

    match variant {
        Variant::Core => (),
        Variant::Filter(f) => opts.filter = Some(f),
        Variant::Query(q) => opts.query = Some(q),
    }

    Ok(opts)
}

fn extract_from_selections(
    raw_snippet: &str,
    contains_key: bool,
) -> Result<(&str, &str, &str), Error> {
    let mut lines = raw_snippet.split('\n');
    let key = if contains_key {
        lines
            .next()
            .context("Key was promised but not present in `selections`")?
    } else {
        "enter"
    };

    let mut parts = lines
        .next()
        .context("No more parts in `selections`")?
        .split(display::DELIMITER)
        .skip(3);

    let tags = parts.next().unwrap_or("");
    parts.next();

    let snippet = parts.next().unwrap_or("");
    Ok((key, tags, snippet))
}

fn prompt_with_suggestions(
    variable_name: &str,
    config: &Config,
    suggestion: &Suggestion,
    _snippet: String,
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

    let opts = suggestion_opts.clone().unwrap_or_default();
    let opts = FinderOpts {
        autoselect: !config.no_autoselect,
        overrides: config.fzf_overrides_var.clone(),
        prompt: Some(display::terminal::variable_prompt(variable_name)),
        ..opts
    };

    let (output, _) = config
        .finder
        .call(opts, |stdin| {
            stdin
                .write_all(suggestions.as_bytes())
                .context("Could not write to finder's stdin")?;
            Ok(None)
        })
        .context("finder was unable to prompt with suggestions")?;

    Ok(output)
}

fn prompt_without_suggestions(variable_name: &str, config: &Config) -> Result<String, Error> {
    let opts = FinderOpts {
        autoselect: false,
        prompt: Some(display::terminal::variable_prompt(variable_name)),
        suggestion_type: SuggestionType::Disabled,
        preview_window: Some("up:1".to_string()),
        ..Default::default()
    };

    let (output, _) = config
        .finder
        .call(opts, |_stdin| Ok(None))
        .context("finder was unable to prompt without suggestions")?;

    Ok(output)
}

fn replace_variables_from_snippet(
    snippet: &str,
    tags: &str,
    variables: VariableMap,
    config: &Config,
) -> Result<String, Error> {
    let mut interpolated_snippet = String::from(snippet);

    for captures in display::VAR_REGEX.captures_iter(snippet) {
        let bracketed_variable_name = &captures[0];
        let variable_name = &bracketed_variable_name[1..bracketed_variable_name.len() - 1];

        let env_value = env::var(variable_name);

        let value = if let Ok(e) = env_value {
            e
        } else {
            variables
                .get_suggestion(&tags, &variable_name)
                .ok_or_else(|| anyhow!("No suggestions"))
                .and_then(|suggestion| {
                    let mut new_suggestion = suggestion.clone();
                    new_suggestion.0 = replace_variables_from_snippet(
                        &new_suggestion.0,
                        tags,
                        variables.clone(),
                        config,
                    )?;

                    prompt_with_suggestions(
                        variable_name,
                        &config,
                        &new_suggestion,
                        interpolated_snippet.clone(),
                    )
                })
                .or_else(|_| prompt_without_suggestions(variable_name, &config))?
        };

        env::set_var(variable_name, &value);

        interpolated_snippet = if value.as_str() == "\n" {
            interpolated_snippet.replacen(bracketed_variable_name, "", 1)
        } else {
            interpolated_snippet.replacen(bracketed_variable_name, value.as_str(), 1)
        };
    }

    Ok(interpolated_snippet)
}

pub fn main(variant: Variant, config: Config, contains_key: bool) -> Result<(), Error> {

    let opts =
        gen_core_finder_opts(variant, &config).context("Failed to generate finder options")?;

    let (raw_selection, variables) = config
        .finder
        .call(opts, |stdin| {
            let mut writer = display::terminal::Writer::new();

            let fetcher: Box<dyn Fetcher> = if config.isTldr() {
                Box::new(tldr::Foo::new())
            } else {
                Box::new(filesystem::Foo::new())
            };

            let res = fetcher
                .fetch(&config, stdin, &mut writer)
                .context("Failed to parse variables intended for finder")?;
            if let Some(variables) = res {
                Ok(Some(variables))
            } else {
                welcome::populate_cheatsheet(&mut writer, stdin);
                Ok(Some(VariableMap::new()))
            }
        })
        .context("Failed getting selection and variables from finder")?;

    let (key, tags, snippet) = extract_from_selections(&raw_selection[..], contains_key)?;

    let interpolated_snippet = display::with_new_lines(
        replace_variables_from_snippet(
            snippet,
            tags,
            variables.expect("No variables received from finder"),
            &config,
        )
        .context("Failed to replace variables from snippet")?,
    );

    // copy to clipboard
    if key == "ctrl-y" {
        clipboard::copy(interpolated_snippet)?;
    // print to stdout
    } else if config.print {
        println!("{}", interpolated_snippet);
    // save to file
    } else if let Some(s) = config.save {
        fs::write(s, interpolated_snippet).context("Unable to save output")?;
    // call navi (this prevents shelling out to call navi again
    } else if interpolated_snippet.starts_with("navi") {
        let new_config = config::config_from_iter(interpolated_snippet.split(' ').collect());
        handler::handle_config(new_config)?;
    // shell out and execute snippet
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(&interpolated_snippet[..])
            .spawn()
            .map_err(|e| BashSpawnError::new(&interpolated_snippet[..], e))?
            .wait()
            .context("bash was not running")?;
    }

    Ok(())
}
