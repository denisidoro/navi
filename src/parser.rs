use crate::display::{self, Writer};
use crate::filesystem;
use crate::structures::cheat::VariableMap;
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use crate::structures::fnv::HashLine;
use crate::structures::option::Command::Alfred;
use crate::structures::{error::filesystem::InvalidPath, item::Item, option::Config};
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

fn parse_opts(text: &str) -> Result<FinderOpts, Error> {
    let mut multi = false;
    let mut prevent_extra = false;
    let mut opts = FinderOpts::default();

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
                    "--map" => opts.map = Some(value.to_string()),
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
                Err(anyhow!("No value provided for the flag `{}`", flag))
            } else {
                unreachable!() // Chunking by 2 allows only for tuples of 1 or 2 items...
            }
        })
        .collect::<Result<_, _>>()
        .context("Failed to parse finder options")?;

    let suggestion_type = match (multi, prevent_extra) {
        (true, _) => SuggestionType::MultipleSelections, // multi wins over prevent-extra
        (false, false) => SuggestionType::SingleRecommendation,
        (false, true) => SuggestionType::SingleSelection,
    };
    opts.suggestion_type = suggestion_type;

    Ok(opts)
}

fn parse_variable_line(line: &str) -> Result<(&str, &str, Option<FinderOpts>), Error> {
    let caps = VAR_LINE_REGEX.captures(line).ok_or_else(|| {
        anyhow!(
            "No variables, command, and options found in the line `{}`",
            line
        )
    })?;
    let variable = caps
        .get(1)
        .ok_or_else(|| anyhow!("No variable captured in the line `{}`", line))?
        .as_str()
        .trim();
    let mut command_plus_opts = caps
        .get(2)
        .ok_or_else(|| anyhow!("No command and options captured in the line `{}`", line))?
        .as_str()
        .split("---");
    let command = command_plus_opts
        .next()
        .ok_or_else(|| anyhow!("No command captured in the line `{}`", line))?;
    let command_options = command_plus_opts.next().map(parse_opts).transpose()?;
    Ok((variable, command, command_options))
}

fn write_cmd(
    tags: &str,
    comment: &str,
    snippet: &str,
    writer: &mut Box<dyn Writer>,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    if snippet.len() <= 1 {
        Ok(())
    } else {
        let item = Item {
            tags: &tags,
            comment: &comment,
            snippet: &snippet,
        };
        stdin
            .write_all(writer.write(item).as_bytes())
            .context("Failed to write command to finder's stdin")
    }
}

fn read_file(
    path: &str,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    writer: &mut Box<dyn Writer>,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");
    let mut should_break = false;

    for (line_nr, line_result) in filesystem::read_lines(path)?.enumerate() {
        let line = line_result
            .with_context(|| format!("Failed to read line nr.{} from `{}`", line_nr, path))?;

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
            if write_cmd(&tags, &comment, &snippet, writer, stdin).is_err() {
                should_break = true
            }
            snippet = String::from("");
            tags = if line.len() > 2 {
                String::from(&line[2..])
            } else {
                String::from("")
            };
        }
        // metacomment
        else if line.starts_with(';') {
        }
        // comment
        else if line.starts_with('#') {
            if write_cmd(&tags, &comment, &snippet, writer, stdin).is_err() {
                should_break = true
            }
            snippet = String::from("");
            comment = if line.len() > 2 {
                String::from(&line[2..])
            } else {
                String::from("")
            };
        }
        // variable
        else if line.starts_with('$') {
            if write_cmd(&tags, &comment, &snippet, writer, stdin).is_err() {
                should_break = true
            }
            snippet = String::from("");
            let (variable, command, opts) = parse_variable_line(&line).with_context(|| {
                format!(
                    "Failed to parse variable line. See line number {} in cheatsheet `{}`",
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
        let _ = write_cmd(&tags, &comment, &snippet, writer, stdin);
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
    let mut writer: Box<dyn Writer> = if let Some(Alfred { cmd: _ }) = &config.cmd {
        Box::new(display::AlfredWriter { is_first: true })
    } else {
        let (tag_width, comment_width) = display::get_widths();
        Box::new(display::FinderWriter {
            tag_width,
            comment_width,
        })
    };
    let paths = filesystem::cheat_paths(config);

    if paths.is_err() {
        welcome::cheatsheet(&mut writer, stdin);
        return Ok(variables);
    }

    let paths = paths.expect("Unable to get paths");
    let folders = paths_from_path_param(&paths);

    for folder in folders {
        if let Ok(dir_entries) = fs::read_dir(folder) {
            for entry in dir_entries {
                if entry.is_ok() {
                    let path = entry.expect("Impossible to read an invalid entry").path();
                    let path_str = path
                        .to_str()
                        .ok_or_else(|| InvalidPath(path.to_path_buf()))?;
                    if path_str.ends_with(".cheat")
                        && read_file(
                            path_str,
                            &mut variables,
                            &mut visited_lines,
                            &mut writer,
                            stdin,
                        )
                        .is_ok()
                        && !found_something
                    {
                        found_something = true;
                    }
                }
            }
        }
    }

    if !found_something {
        welcome::cheatsheet(&mut writer, stdin);
    }

    Ok(variables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_line() {
        let (variable, command, command_options) =
            parse_variable_line("$ user : echo -e \"$(whoami)\\nroot\" --- --prevent-extra")
                .unwrap();
        assert_eq!(command, " echo -e \"$(whoami)\\nroot\" ");
        assert_eq!(variable, "user");
        assert_eq!(
            command_options,
            Some(FinderOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleSelection,
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
        let mut writer: Box<dyn Writer> = Box::new(display::FinderWriter {comment_width: 20, tag_width: 30});
        read_file(path, &mut variables, &mut visited_lines, &mut writer, child_stdin).unwrap();
        let expected_suggestion = (
            r#" echo -e "$(whoami)\nroot" "#.to_string(),
            Some(FinderOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleSelection,
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
