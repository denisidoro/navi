use crate::common::hash::fnv;
use crate::display::{self, Writer};
use crate::structures::cheat::VariableMap;
use crate::structures::finder::{Opts as FinderOpts, SuggestionType};
use crate::structures::item::Item;
use anyhow::{Context, Error};
use regex::Regex;
use std::collections::HashSet;
use std::io::Write;

lazy_static! {
    pub static ref VAR_LINE_REGEX: Regex = Regex::new(r"^\$\s*([^:]+):(.*)").expect("Invalid regex");
}

fn parse_opts(text: &str) -> Result<FinderOpts, Error> {
    let mut multi = false;
    let mut prevent_extra = false;
    let mut is_global = false;
    let mut opts = FinderOpts::default();

    let parts = shellwords::split(text).map_err(|_| anyhow!("Given options are missing a closing quote"))?;

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
                "--global" => {
                    is_global = true;
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
                    "--headers" | "--header-lines" => opts.header_lines = value.parse::<u8>().context("Value for `--headers` is invalid u8")?,
                    "--column" => opts.column = Some(value.parse::<u8>().context("Value for `--column` is invalid u8")?),
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
    opts.global = is_global;

    Ok(opts)
}

fn parse_variable_line(line: &str) -> Result<(&str, &str, Option<FinderOpts>), Error> {
    let caps = VAR_LINE_REGEX
        .captures(line)
        .ok_or_else(|| anyhow!("No variables, command, and options found in the line `{}`", line))?;
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
    file_index: &usize,
    writer: &mut dyn Writer,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    if snippet.len() <= 1 {
        Ok(())
    } else {
        let item = Item {
            tags: &tags,
            comment: &comment,
            snippet: &snippet,
            file_index: &file_index,
        };
        stdin
            .write_all(writer.write(item).as_bytes())
            .context("Failed to write command to finder's stdin")
    }
}

fn without_prefix(line: &str) -> String {
    if line.len() > 2 {
        String::from(line[2..].trim())
    } else {
        String::from("")
    }
}

pub fn read_lines(
    lines: impl Iterator<Item = Result<String, Error>>,
    id: &str,
    file_index: usize,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    writer: &mut dyn Writer,
    stdin: &mut std::process::ChildStdin,
) -> Result<(), Error> {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");
    let mut should_break = false;

    for (line_nr, line_result) in lines.enumerate() {
        let line = line_result.with_context(|| format!("Failed to read line number {} in cheatsheet `{}`", line_nr, id))?;

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
            should_break = write_cmd(&tags, &comment, &snippet, &file_index, writer, stdin).is_err();
            snippet = String::from("");
            tags = without_prefix(&line);
        }
        // dependency
        else if line.starts_with('@') {
            let tags_dependency = without_prefix(&line);
            variables.insert_dependency(&tags, &tags_dependency);
        }
        // metacomment
        else if line.starts_with(';') {
        }
        // comment
        else if line.starts_with('#') {
            should_break = write_cmd(&tags, &comment, &snippet, &file_index, writer, stdin).is_err();
            snippet = String::from("");
            comment = without_prefix(&line);
        }
        // variable
        else if line.starts_with('$') {
            should_break = write_cmd(&tags, &comment, &snippet, &file_index, writer, stdin).is_err();
            snippet = String::from("");
            let (variable, command, opts) = parse_variable_line(&line)
                .with_context(|| format!("Failed to parse variable line. See line number {} in cheatsheet `{}`", line_nr + 1, id))?;
            variables.insert_suggestion(&tags, &variable, (String::from(command), opts));
        }
        // snippet
        else {
            let hash = fnv(&format!("{}{}", &comment, &line));
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
        let _ = write_cmd(&tags, &comment, &snippet, &file_index, writer, stdin);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_line() {
        let (variable, command, command_options) = parse_variable_line("$ user : echo -e \"$(whoami)\\nroot\" --- --prevent-extra").unwrap();
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
}
