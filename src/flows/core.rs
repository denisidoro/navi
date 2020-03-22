use crate::display;
use crate::filesystem;
use crate::flows;
use crate::fzf;
use crate::handler;
use crate::parser;
use crate::structures::cheat::{Suggestion, VariableMap};
use crate::structures::fzf::{Opts as FzfOpts, SuggestionType};
use crate::structures::option;
use crate::structures::option::Config;
use anyhow::Context;
use anyhow::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

lazy_static! {
    pub static ref VAR_REGEX: Regex = Regex::new(r"<(\w[\w\d\-_]*)>").expect("Invalid regex");
}

pub enum Variant {
    Core,
    Filter(String),
    Query(String),
}

fn gen_core_fzf_opts(variant: Variant, config: &Config) -> Result<FzfOpts, Error> {
    let mut opts = FzfOpts {
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

fn extract_from_selections(raw_snippet: &str, contains_key: bool) -> (&str, &str, &str) {
    let mut lines = raw_snippet.split('\n');
    let key = if contains_key {
        lines
            .next()
            .expect("Key was promised but not present in `selections`")
    } else {
        "enter"
    };

    let mut parts = lines
        .next()
        .expect("No more parts in `selections`")
        .split(display::DELIMITER)
        .skip(3);

    let tags = parts.next().unwrap_or("");
    parts.next();

    let snippet = parts.next().unwrap_or("");
    (key, tags, snippet)
}

fn prompt_with_suggestions(
    varname: &str,
    config: &Config,
    suggestion: &Suggestion,
    values: &HashMap<String, String>,
) -> Result<String, Error> {
    let mut vars_cmd = String::from("");
    for (key, value) in values.iter() {
        vars_cmd.push_str(format!("{}=\"{}\"; ", key, value).as_str());
    }
    let (suggestion_command, suggestion_opts) = suggestion;
    let command = format!("{} {}", vars_cmd, suggestion_command);

    let child = Command::new("bash")
        .stdout(Stdio::piped())
        .arg("-c")
        .arg(command)
        .spawn()
        .context("Failed to execute bash")?;

    let suggestions = String::from_utf8(
        child
            .wait_with_output()
            .context("Failed to wait and collect output from bash")?
            .stdout,
    )
    .context("Suggestions are invalid utf8")?;

    let opts = suggestion_opts.clone().unwrap_or_default();
    let opts = FzfOpts {
        autoselect: !config.no_autoselect,
        overrides: config.fzf_overrides_var.clone(),
        prompt: Some(display::variable_prompt(varname)),
        ..opts
    };

    let (output, _) = fzf::call(opts, |stdin| {
        stdin
            .write_all(suggestions.as_bytes())
            .context("Could not write to fzf's stdin")?;
        Ok(None)
    })
    .context("fzf was unable to prompt with suggestions")?;

    Ok(output)
}

fn prompt_without_suggestions(variable_name: &str) -> Result<String, Error> {
    let opts = FzfOpts {
        autoselect: false,
        prompt: Some(display::variable_prompt(variable_name)),
        suggestion_type: SuggestionType::Disabled,
        ..Default::default()
    };

    let (output, _) = fzf::call(opts, |_stdin| Ok(None))
        .context("fzf was unable to prompt without suggestions")?;

    Ok(output)
}

fn replace_variables_from_snippet(
    snippet: &str,
    tags: &str,
    variables: VariableMap,
    config: &Config,
) -> Result<String, Error> {
    let mut interpolated_snippet = String::from(snippet);
    let mut values: HashMap<String, String> = HashMap::new();

    for captures in VAR_REGEX.captures_iter(snippet) {
        let bracketed_variable_name = &captures[0];
        let variable_name = &bracketed_variable_name[1..bracketed_variable_name.len() - 1];

        let value = values
            .get(variable_name)
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!(format!("No value for variable `{}`", variable_name)))
            .or_else(|_| {
                variables
                    .get(&tags, &variable_name)
                    .ok_or_else(|| anyhow!("No suggestions"))
                    .and_then(|suggestion| {
                        prompt_with_suggestions(variable_name, &config, suggestion, &values)
                    })
                    .or_else(|_| prompt_without_suggestions(variable_name))
            })?;

        values.insert(variable_name.to_string(), value.clone());

        interpolated_snippet =
            interpolated_snippet.replacen(bracketed_variable_name, value.as_str(), 1);
    }

    Ok(interpolated_snippet)
}

fn with_new_lines(txt: String) -> String {
    txt.replace(display::LINE_SEPARATOR, "\n")
}

pub fn main(variant: Variant, config: Config, contains_key: bool) -> Result<(), Error> {
    let _ = display::WIDTHS;

    let opts = gen_core_fzf_opts(variant, &config).context("Failed to generate fzf options")?;
    let (raw_selection, variables) = fzf::call(opts, |stdin| {
        Ok(Some(
            parser::read_all(&config, stdin)
                .context("Failed to parse variables intended for fzf")?,
        ))
    })
    .context("Failed getting selection and variables from fzf")?;

    let (key, tags, snippet) = extract_from_selections(&raw_selection[..], contains_key);

    let interpolated_snippet = with_new_lines(
        replace_variables_from_snippet(
            snippet,
            tags,
            variables.expect("No variables received from fzf"),
            &config,
        )
        .context("Failed to replace variables from snippet")?,
    );

    // copy to clipboard
    if key == "ctrl-y" {
        flows::aux::abort("copying snippets to the clipboard", 201)?
    // print to stdout
    } else if config.print {
        println!("{}", interpolated_snippet);
    // save to file
    } else if let Some(s) = config.save {
        fs::write(s, interpolated_snippet).context("Unable to save config")?;
    // call navi (this prevents "failed to read /dev/tty" from fzf)
    } else if interpolated_snippet.starts_with("navi") {
        let new_config = option::config_from_iter(interpolated_snippet.split(' ').collect());
        handler::handle_config(new_config)?;
    // shell out and execute snippet
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(&interpolated_snippet[..])
            .spawn()
            .context("Failed to execute bash")?
            .wait()
            .context("bash was not running")?;
    }

    Ok(())
}
