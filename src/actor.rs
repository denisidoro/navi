use crate::clipboard;
use crate::env_var;
use crate::extractor;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::Finder;
use crate::shell;
use crate::shell::{ShellSpawnError, IS_FISH};
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::config::Action;
use crate::structures::config::Config;
use crate::writer;
use anyhow::Context;
use anyhow::Error;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn prompt_finder(
    variable_name: &str,
    config: &Config,
    suggestion: Option<&Suggestion>,
    variable_count: usize,
) -> Result<String, Error> {
    env_var::remove(env_var::PREVIEW_COLUMN);
    env_var::remove(env_var::PREVIEW_DELIMITER);
    env_var::remove(env_var::PREVIEW_MAP);

    let mut extra_preview = None;

    let (suggestions, initial_opts) = if let Some(s) = suggestion {
        let (suggestion_command, suggestion_opts) = s;

        if let Some(sopts) = suggestion_opts {
            if let Some(c) = &sopts.column {
                env_var::set(env_var::PREVIEW_COLUMN, c.to_string());
            }
            if let Some(d) = &sopts.delimiter {
                env_var::set(env_var::PREVIEW_DELIMITER, d);
            }
            if let Some(m) = &sopts.map {
                env_var::set(env_var::PREVIEW_MAP, m);
            }
            if let Some(p) = &sopts.preview {
                extra_preview = Some(format!(";echo;{}", p));
            }
        }

        let child = shell::command()
            .stdout(Stdio::piped())
            .arg("-c")
            .arg(&suggestion_command)
            .spawn()
            .map_err(|e| ShellSpawnError::new(suggestion_command, e))?;

        let text = String::from_utf8(
            child
                .wait_with_output()
                .context("Failed to wait and collect output from bash")?
                .stdout,
        )
        .context("Suggestions are invalid utf8")?;

        (text, suggestion_opts)
    } else {
        ('\n'.to_string(), &None)
    };

    let overrides = {
        let mut o = config.fzf_overrides.clone();
        if let Some(io) = initial_opts {
            if io.overrides.is_some() {
                o = io.overrides.clone()
            }
        }
        o
    };

    let mut opts = FinderOpts {
        overrides,
        preview: Some(format!(
            r#"{prefix}navi preview-var "$(cat <<NAVIEOF
{{+}}
NAVIEOF
)" "$(cat <<NAVIEOF
{{q}}
NAVIEOF
)" "{name}"; {extra}{suffix}"#,
            prefix = if *IS_FISH { "bash -c '" } else { "" },
            suffix = if *IS_FISH { "'" } else { "" },
            name = variable_name,
            extra = extra_preview.clone().unwrap_or_default()
        )),
        ..initial_opts.clone().unwrap_or_default()
    };

    opts.query = env_var::get(format!("{}__query", variable_name)).ok();

    if let Ok(f) = env_var::get(format!("{}__best", variable_name)) {
        opts.filter = Some(f);
        opts.suggestion_type = SuggestionType::SingleSelection;
    }

    if opts.preview_window.is_none() {
        opts.preview_window = Some(if extra_preview.is_none() {
            format!("up:{}", variable_count + 3)
        } else {
            "right:50%".to_string()
        });
    }

    if suggestion.is_none() {
        opts.suggestion_type = SuggestionType::Disabled;
    };

    let (output, _, _) = config
        .finder
        .call(opts, |stdin, _| {
            stdin
                .write_all(suggestions.as_bytes())
                .context("Could not write to finder's stdin")?;
            Ok(None)
        })
        .context("finder was unable to prompt with suggestions")?;

    Ok(output)
}

fn unique_result_count(results: &[&str]) -> usize {
    let mut vars = results.to_owned();
    vars.sort_unstable();
    vars.dedup();
    vars.len()
}

fn replace_variables_from_snippet(
    snippet: &str,
    tags: &str,
    variables: VariableMap,
    config: &Config,
) -> Result<String, Error> {
    let mut interpolated_snippet = String::from(snippet);
    let variables_found: Vec<&str> = writer::VAR_REGEX.find_iter(snippet).map(|m| m.as_str()).collect();
    let variable_count = unique_result_count(&variables_found);

    for bracketed_variable_name in variables_found {
        let variable_name = &bracketed_variable_name[1..bracketed_variable_name.len() - 1];

        let env_variable_name = env_var::escape(variable_name);
        let env_value = env_var::get(&env_variable_name);

        let value = if let Ok(e) = env_value {
            e
        } else if let Some(suggestion) = variables.get_suggestion(&tags, &variable_name) {
            let mut new_suggestion = suggestion.clone();
            new_suggestion.0 =
                replace_variables_from_snippet(&new_suggestion.0, tags, variables.clone(), config)?;
            prompt_finder(variable_name, &config, Some(&new_suggestion), variable_count)?
        } else {
            prompt_finder(variable_name, &config, None, variable_count)?
        };

        env_var::set(env_variable_name, &value);

        interpolated_snippet = if value.as_str() == "\n" {
            interpolated_snippet.replacen(bracketed_variable_name, "", 1)
        } else {
            interpolated_snippet.replacen(bracketed_variable_name, value.as_str(), 1)
        };
    }

    Ok(interpolated_snippet)
}

// TODO: make it depend on less inputs
pub fn act(
    extractions: Result<extractor::Output, Error>,
    config: Config,
    files: Vec<String>,
    variables: Option<VariableMap>,
) -> Result<(), Error> {
    let (key, tags, comment, snippet, file_index) = extractions.unwrap();

    if key == "ctrl-o" {
        edit::edit_file(Path::new(&files[file_index.expect("No files found")]))
            .expect("Cound not open file in external editor");
        return Ok(());
    }

    env_var::set(env_var::PREVIEW_INITIAL_SNIPPET, &snippet);
    env_var::set(env_var::PREVIEW_TAGS, &tags);
    env_var::set(env_var::PREVIEW_COMMENT, &comment);

    let interpolated_snippet = writer::with_new_lines(
        replace_variables_from_snippet(
            snippet,
            tags,
            variables.expect("No variables received from finder"),
            &config,
        )
        .context("Failed to replace variables from snippet")?,
    );

    match config.action() {
        Action::Print => {
            println!("{}", interpolated_snippet);
        }
        Action::Execute => match key {
            "ctrl-y" => {
                clipboard::copy(interpolated_snippet)?;
            }
            _ => {
                shell::command()
                    .arg("-c")
                    .arg(&interpolated_snippet[..])
                    .spawn()
                    .map_err(|e| ShellSpawnError::new(&interpolated_snippet[..], e))?
                    .wait()
                    .context("bash was not running")?;
            }
        },
    };

    Ok(())
}
