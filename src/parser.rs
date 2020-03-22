use crate::display;
use crate::filesystem;
use crate::structures::cheat::VariableMap;
use crate::structures::fnv::HashLine;
use crate::structures::fzf::{Opts as FzfOpts, SuggestionType};
use crate::structures::option::Config;
use crate::welcome;
use anyhow::{Context, Error};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::Write;

lazy_static! {
    pub static ref VAR_LINE_REGEX: Regex =
        Regex::new(r"^\$\s*([^:]+):(.*)").expect("Invalid regex");
}

fn parse_opts(text: &str) -> Result<FzfOpts, Error> {
    let mut multi = false;
    let mut prevent_extra = false;
    let mut opts = FzfOpts::default();
    let parts = shellwords::split(text)
        .map_err(|_| anyhow!("Given options are missing a closing quote"))?;

    parts
        .into_iter()
        .filter(|part| {
            // We'll take parts in pairs of 2: (argument, value). Flags don't have a value tho, so we filter and handle them beforehand.
            match part.as_str() {
                "--multi" => {
                    multi = true;
                    false
                }
                "--prevent-extra" => {
                    prevent_extra = true;
                    false
                }
                _ => true,
            }
        })
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|flag_and_value| {
            if let [flag, value] = flag_and_value {
                match flag.as_str() {
                    "--headers" | "--header-lines" => {
                        opts.header_lines = value
                            .parse::<u8>()
                            .context("Value for `--headers` is invalid u8")?
                    }
                    "--column" => {
                        opts.column = Some(
                            value
                                .parse::<u8>()
                                .context("Value for `--column` is invalid u8")?,
                        )
                    }
                    "--delimiter" => opts.delimiter = Some(value.to_string()),
                    "--query" => opts.query = Some(value.to_string()),
                    "--filter" => opts.filter = Some(value.to_string()),
                    "--preview" => opts.preview = Some(value.to_string()),
                    "--preview-window" => opts.preview_window = Some(value.to_string()),
                    "--header" => opts.header = Some(value.to_string()),
                    "--overrides" => opts.overrides = Some(value.to_string()),
                    _ => (),
                }
                Ok(())
            } else if let [flag] = flag_and_value {
                Err(anyhow!("No value provided for the flag {}", flag))
            } else {
                unreachable!() // Chunking by 2 allows only for tuples of 1 or 2 items...
            }
        })
        .collect::<Result<_, _>>()
        .context("Failed to parse fzf options")?;

    let suggestion_type = match (multi, prevent_extra) {
        (true, _) => SuggestionType::MultipleSelections, // multi wins over allow-extra
        (false, false) => SuggestionType::SingleRecommendation,
        (false, true) => SuggestionType::SingleSelection,
    };
    opts.suggestion_type = suggestion_type;

    Ok(opts)
}

fn parse_variable_line(line: &str) -> Result<(&str, &str, Option<FzfOpts>), Error> {
    let caps = VAR_LINE_REGEX.captures(line).ok_or_else(|| {
        anyhow!(
            "No variables, command, and options found in the line {}",
            line
        )
    })?;
    let variable = caps
        .get(1)
        .ok_or_else(|| anyhow!("No variable captured in the line {}", line))?
        .as_str()
        .trim();
    let mut command_plus_opts = caps
        .get(2)
        .ok_or_else(|| anyhow!("No command and options captured in the line {}", line))?
        .as_str()
        .split("---");
    let command = command_plus_opts
        .next()
        .ok_or_else(|| anyhow!("No command captured in the line {}", line))?;
    let command_options = command_plus_opts.next().map(parse_opts).transpose()?;
    Ok((variable, command, command_options))
}

fn write_cmd(
    tags: &str,
    comment: &str,
    snippet: &str,
    tag_width: usize,
    comment_width: usize,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    if snippet.is_empty() {
        Ok(())
    } else {
        stdin
            .write_all(
                display::format_line(&tags, &comment, &snippet, tag_width, comment_width)
                    .as_bytes(),
            )
            .context("Failed to write command to fzf's stdin")
    }
}

fn read_file(
    path: &str,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");
    let mut should_break = false;

    let (tag_width, comment_width) = *display::WIDTHS;

    for (line_nr, line) in filesystem::read_lines(path)?.into_iter().enumerate() {
        if should_break {
            break;
        }

        // duplicate
        if !tags.is_empty() && !comment.is_empty() {}

        // blank
        if line.is_empty() {
        }
        // tag
        else if line.starts_with('%') {
            if write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin).is_err() {
                should_break = true
            }
            snippet = String::from("");
            tags = String::from(&line[2..]);
        }
        // metacomment
        else if line.starts_with(';') {
        }
        // comment
        else if line.starts_with('#') {
            if write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin).is_err() {
                should_break = true
            }
            snippet = String::from("");
            comment = String::from(&line[2..]);
        }
        // variable
        else if line.starts_with('$') {
            if write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin).is_err() {
                should_break = true
            }
            snippet = String::from("");
            let (variable, command, opts) = parse_variable_line(&line).with_context(|| {
                format!(
                    "Failed to parse variable line. See line nr.{} in cheatsheet {}",
                    line_nr + 1,
                    path
                )
            })?;
            variables.insert(&tags, &variable, (String::from(command), opts));
        }
        // snippet
        else {
            let hash = format!("{}{}", &comment, &line).hash_line();
            if visited_lines.contains(&hash) {
                continue;
            }
            visited_lines.insert(hash);

            if !(&snippet).is_empty() {
                snippet.push_str(display::LINE_SEPARATOR);
            }
            snippet.push_str(&line);
        }
    }

    if !should_break {
        let _ = write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin);
    }

    Ok(())
}

fn paths_from_path_param<'a>(env_var: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    env_var.split(':').filter(|folder| folder != &"")
}

pub fn read_all(
    config: &Config,
    stdin: &mut std::process::ChildStdin,
) -> Result<VariableMap, Error> {
    let mut variables = VariableMap::new();
    let mut found_something = false;
    let mut visited_lines = HashSet::new();
    let paths = filesystem::cheat_paths(config)?;
    let folders = paths_from_path_param(&paths);

    for folder in folders {
        let dir_entries =
            fs::read_dir(folder).with_context(|| format!("Unable to read directory {}", folder))?;

        for entry in dir_entries {
            let path = entry
                .with_context(|| format!("Unable to read directory {}", folder))?
                .path();
            let path_str = path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid path {}", path.display()))?;
            if path_str.ends_with(".cheat")
                && read_file(path_str, &mut variables, &mut visited_lines, stdin).is_ok()
                && !found_something
            {
                found_something = true;
            }
        }
    }

    if !found_something {
        welcome::cheatsheet(stdin);
    }

    Ok(variables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_line() {
        let (variable, command, command_options) =
            parse_variable_line("$ user : echo -e \"$(whoami)\\nroot\" --- --allow-extra").unwrap();
        assert_eq!(command, " echo -e \"$(whoami)\\nroot\" ");
        assert_eq!(variable, "user");
        assert_eq!(
            command_options,
            Some(FzfOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleRecommendation,
                ..Default::default()
            })
        );
    }
    use std::process::{Command, Stdio};

    #[test]
    fn test_read_file() {
        let path = "tests/cheats/ssh.cheat";
        let mut variables = VariableMap::new();
        let mut child = Command::new("cat").stdin(Stdio::piped()).spawn().unwrap();
        let child_stdin = child.stdin.as_mut().unwrap();
        let mut visited_lines: HashSet<u64> = HashSet::new();
        read_file(path, &mut variables, &mut visited_lines, child_stdin).unwrap();
        let expected_suggestion = (
            r#" echo -e "$(whoami)\nroot" "#.to_string(),
            Some(FzfOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleRecommendation,
                ..Default::default()
            }),
        );
        let actual_suggestion = variables.get("ssh", "user");
        assert_eq!(Some(&expected_suggestion), actual_suggestion);
    }

    #[test]
    fn splitting_of_dirs_param_may_not_contain_empty_items() {
        // Trailing colon indicates potential extra path. Split returns an empty item for it. This empty item should be filtered away, which is what this test checks.
        let given_path_config = "SOME_PATH:ANOTHER_PATH:";

        let found_paths = paths_from_path_param(given_path_config);

        let mut expected_paths = vec!["SOME_PATH", "ANOTHER_PATH"].into_iter();

        for found in found_paths {
            let expected = expected_paths.next().unwrap();
            assert_eq!(found, expected)
        }
    }
}
