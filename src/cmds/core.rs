use crate::cheatsh;
use crate::common::clipboard;
use crate::common::shell::{BashSpawnError, IS_FISH};
use crate::display;
use crate::env_vars;
use crate::fetcher::Fetcher;
use crate::filesystem;
use crate::finder::Finder;
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::config::Action;
use crate::structures::config::Config;
use crate::structures::config::Source;
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
        autoselect: config.autoselect(),
        overrides: config.fzf_overrides.clone(),
        suggestion_type: SuggestionType::SnippetSelection,
        query: if config.get_best_match() { None } else { config.get_query() },
        filter: if config.get_best_match() { config.get_query() } else { None },
        ..Default::default()
    };

    Ok(opts)
}

fn extract_from_selections(raw_snippet: &str, is_single: bool) -> Result<(&str, &str, &str, &str), Error> {
    let mut lines = raw_snippet.split('\n');
    let key = if is_single {
        "enter"
    } else {
        lines.next().context("Key was promised but not present in `selections`")?
    };

    let mut parts = lines.next().context("No more parts in `selections`")?.split(display::DELIMITER).skip(3);

    let tags = parts.next().unwrap_or("");
    let comment = parts.next().unwrap_or("");
    let snippet = parts.next().unwrap_or("");
    Ok((key, tags, comment, snippet))
}

fn prompt_finder(variable_name: &str, config: &Config, suggestion: Option<&Suggestion>, variable_count: usize) -> Result<String, Error> {
    env::remove_var(env_vars::PREVIEW_COLUMN);
    env::remove_var(env_vars::PREVIEW_DELIMITER);
    env::remove_var(env_vars::PREVIEW_MAP);

    let mut extra_preview = None;

    let (suggestions, opts) = if let Some(s) = suggestion {
        let (suggestion_command, suggestion_opts) = s;

        if let Some(sopts) = suggestion_opts {
            if let Some(c) = &sopts.column {
                env::set_var(env_vars::PREVIEW_COLUMN, c.to_string());
            }
            if let Some(d) = &sopts.delimiter {
                env::set_var(env_vars::PREVIEW_DELIMITER, d);
            }
            if let Some(m) = &sopts.map {
                env::set_var(env_vars::PREVIEW_MAP, m);
            }
            if let Some(p) = &sopts.preview {
                extra_preview = Some(format!(";echo;{}", p));
            }
        }

        let child = Command::new("bash")
            .stdout(Stdio::piped())
            .arg("-c")
            .arg(&suggestion_command)
            .spawn()
            .map_err(|e| BashSpawnError::new(suggestion_command, e))?;

        let text = String::from_utf8(child.wait_with_output().context("Failed to wait and collect output from bash")?.stdout)
            .context("Suggestions are invalid utf8")?;

        (text, suggestion_opts)
    } else {
        ('\n'.to_string(), &None)
    };

    let mut opts = FinderOpts {
        autoselect: config.autoselect(),
        overrides: config.fzf_overrides_var.clone(),
        preview: Some(format!(
            r#"{prefix}navi preview-var "$(cat <<NAVIEOF
{{}}
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
        ..opts.clone().unwrap_or_default()
    };

    opts.query = env::var(format!("{}__query", variable_name)).ok();

    if let Ok(f) = env::var(format!("{}__best", variable_name)) {
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

    let (output, _) = config
        .finder
        .call(opts, |stdin| {
            stdin.write_all(suggestions.as_bytes()).context("Could not write to finder's stdin")?;
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

fn replace_variables_from_snippet(snippet: &str, tags: &str, variables: VariableMap, config: &Config) -> Result<String, Error> {
    let mut interpolated_snippet = String::from(snippet);
    let variables_found: Vec<&str> = display::VAR_REGEX.find_iter(snippet).map(|m| m.as_str()).collect();
    let variable_count = unique_result_count(&variables_found);

    for bracketed_variable_name in variables_found {
        let variable_name = &bracketed_variable_name[1..bracketed_variable_name.len() - 1];

        let env_value = env::var(variable_name);

        let value = if let Ok(e) = env_value {
            e
        } else if let Some(suggestion) = variables.get_suggestion(&tags, &variable_name) {
            let mut new_suggestion = suggestion.clone();
            new_suggestion.0 = replace_variables_from_snippet(&new_suggestion.0, tags, variables.clone(), config)?;
            prompt_finder(variable_name, &config, Some(&new_suggestion), variable_count)?
        } else {
            prompt_finder(variable_name, &config, None, variable_count)?
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
                Source::CHEATSH(query) => Box::new(cheatsh::Fetcher::new(query)),
                Source::TLDR(query) => Box::new(tldr::Fetcher::new(query)),
                Source::FILESYSTEM(path) => Box::new(filesystem::Fetcher::new(path)),
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

    let (key, tags, comment, snippet) = extract_from_selections(&raw_selection, config.get_best_match())?;

    env::set_var(env_vars::PREVIEW_INITIAL_SNIPPET, &snippet);
    env::set_var(env_vars::PREVIEW_TAGS, &tags);
    env::set_var(env_vars::PREVIEW_COMMENT, &comment);

    let interpolated_snippet = display::with_new_lines(
        replace_variables_from_snippet(snippet, tags, variables.expect("No variables received from finder"), &config)
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
