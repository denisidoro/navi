use crate::common::clipboard;
use crate::common::shell::BashSpawnError;
use crate::display;
use crate::fetcher::Fetcher;
use crate::filesystem;
use crate::finder::Finder;
use crate::structures::config::Source;
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::config::Action;
use crate::structures::config::Config;
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use crate::tldr;
use crate::welcome;
use anyhow::Context;
use anyhow::Error;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn gen_core_finder_opts(config: &Config) -> Result<FinderOpts, Error> {
    let opts = FinderOpts {
        preview: if config.no_preview {
            None
        } else {
            Some(format!("{} preview {{}}", filesystem::exe_string()?))
        },
        autoselect: !config.get_no_autoselect(),
        overrides: config.fzf_overrides.clone(),
        suggestion_type: SuggestionType::SnippetSelection,
        query: if config.get_single() {
            None
        } else {
            config.get_query()
        },
        filter: if config.get_single() {
            config.get_query()
        } else {
            None
        },
        ..Default::default()
    };

    Ok(opts)
}

fn extract_from_selections(
    raw_snippet: &str,
    is_single: bool,
) -> Result<(&str, &str, &str), Error> {
    let mut lines = raw_snippet.split('\n');
    let key = if !is_single {
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
        autoselect: !config.get_no_autoselect(),
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

pub fn main(config: Config) -> Result<(), Error> {
    let opts = gen_core_finder_opts(&config).context("Failed to generate finder options")?;

    let (raw_selection, variables) = config
        .finder
        .call(opts, |stdin| {
            let mut writer = display::terminal::Writer::new();

            let fetcher: Box<dyn Fetcher> = match config.source() {
                Source::TLDR(query) => Box::new(tldr::Foo::new(query)),
                Source::FILESYSTEM(path) => Box::new(filesystem::Foo::new(path)),
            };

            let res = fetcher
                .fetch(stdin, &mut writer)
                .context("Failed to parse variables intended for finder")?;

            if let Some(variables) = res {
                Ok(Some(variables))
            } else {
                welcome::populate_cheatsheet(&mut writer, stdin);
                Ok(Some(VariableMap::new()))
            }
        })
        .context("Failed getting selection and variables from finder")?;

    let (key, tags, snippet) = extract_from_selections(&raw_selection[..], config.get_single())?;

    let interpolated_snippet = display::with_new_lines(
        replace_variables_from_snippet(
            snippet,
            tags,
            variables.expect("No variables received from finder"),
            &config,
        )
        .context("Failed to replace variables from snippet")?,
    );

    match config.action() {
        Action::PRINT => {
            println!("{}", interpolated_snippet);
        }
        Action::SAVE(filepath) => {
            fs::write(filepath, interpolated_snippet).context("Unable to save output")?;
        }
        Action::EXECUTE => {
            if key == "ctrl-y" {
                clipboard::copy(interpolated_snippet)?;
            } else {
                Command::new("bash")
                    .arg("-c")
                    .arg(&interpolated_snippet[..])
                    .spawn()
                    .map_err(|e| BashSpawnError::new(&interpolated_snippet[..], e))?
                    .wait()
                    .context("bash was not running")?;
            }
        }
    };

    Ok(())
}
