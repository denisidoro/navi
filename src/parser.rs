use crate::config::CONFIG;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::hash::fnv;
use crate::structures::cheat::VariableMap;
use crate::structures::item::Item;
use crate::writer;
use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::io::Write;

lazy_static! {
    pub static ref VAR_LINE_REGEX: Regex = Regex::new(r"^\$\s*([^:]+):(.*)").expect("Invalid regex");
}

fn parse_opts(text: &str) -> Result<FinderOpts> {
    let mut multi = false;
    let mut prevent_extra = false;

    let mut opts = FinderOpts {
        overrides: CONFIG.fzf_overrides_var(),
        suggestion_type: SuggestionType::SingleRecommendation,
        query: None,
        filter: None,
        prevent_select1: false,
        ..Default::default()
    };

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
                _ => true,
            }
        })
        .collect::<Vec<_>>()
        .chunks(2)
        .try_for_each(|flag_and_value| {
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
                    "--fzf-overrides" => opts.overrides = Some(value.to_string()),
                    _ => (),
                }
                Ok(())
            } else if let [flag] = flag_and_value {
                Err(anyhow!("No value provided for the flag `{}`", flag))
            } else {
                unreachable!() // Chunking by 2 allows only for tuples of 1 or 2 items...
            }
        })
        .context("Failed to parse finder options")?;

    let suggestion_type = match (multi, prevent_extra) {
        (true, _) => SuggestionType::MultipleSelections, // multi wins over prevent-extra
        (false, false) => SuggestionType::SingleRecommendation,
        (false, true) => SuggestionType::SingleSelection,
    };
    opts.suggestion_type = suggestion_type;

    Ok(opts)
}

fn parse_variable_line(line: &str) -> Result<(&str, &str, Option<FinderOpts>)> {
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
    item: &Item,
    stdin: &mut std::process::ChildStdin,
    allowlist: Option<&Vec<String>>,
    denylist: Option<&Vec<String>>,
) -> Result<()> {
    if item.snippet.len() <= 1 {
        return Ok(());
    }

    if let Some(list) = denylist {
        for v in list {
            if item.tags.contains(v) {
                return Ok(());
            }
        }
    }

    if let Some(list) = allowlist {
        let mut should_allow = false;
        for v in list {
            if item.tags.contains(v) {
                should_allow = true;
                break;
            }
        }
        if !should_allow {
            return Ok(());
        }
    }

    return stdin
        .write_all(writer::write(item).as_bytes())
        .context("Failed to write command to finder's stdin");
}

fn without_prefix(line: &str) -> String {
    if line.len() > 2 {
        String::from(line[2..].trim())
    } else {
        String::from("")
    }
}

#[allow(clippy::too_many_arguments)]
pub fn read_lines(
    lines: impl Iterator<Item = Result<String>>,
    id: &str,
    file_index: usize,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    stdin: &mut std::process::ChildStdin,
    allowlist: Option<&Vec<String>>,
    denylist: Option<&Vec<String>>,
) -> Result<()> {
    let mut item = Item::new();
    item.file_index = file_index;

    let mut should_break = false;

    for (line_nr, line_result) in lines.enumerate() {
        let line = line_result
            .with_context(|| format!("Failed to read line number {} in cheatsheet `{}`", line_nr, id))?;

        if should_break {
            break;
        }

        // duplicate
        if !item.tags.is_empty() && !item.comment.is_empty() {}
        // blank
        if line.is_empty() {
        }
        // tag
        else if line.starts_with('%') {
            should_break = write_cmd(&item, stdin, allowlist, denylist).is_err();
            item.snippet = String::from("");
            item.tags = without_prefix(&line);
        }
        // dependency
        else if line.starts_with('@') {
            let tags_dependency = without_prefix(&line);
            variables.insert_dependency(&item.tags, &tags_dependency);
        }
        // metacomment
        else if line.starts_with(';') {
        }
        // comment
        else if line.starts_with('#') {
            should_break = write_cmd(&item, stdin, allowlist, denylist).is_err();
            item.snippet = String::from("");
            item.comment = without_prefix(&line);
        }
        // variable
        else if line.starts_with('$') {
            should_break = write_cmd(&item, stdin, allowlist, denylist).is_err();
            item.snippet = String::from("");
            let (variable, command, opts) = parse_variable_line(&line).with_context(|| {
                format!(
                    "Failed to parse variable line. See line number {} in cheatsheet `{}`",
                    line_nr + 1,
                    id
                )
            })?;
            variables.insert_suggestion(&item.tags, variable, (String::from(command), opts));
        }
        // snippet
        else {
            let hash = fnv(&format!("{}{}", &item.comment, &line));
            if visited_lines.contains(&hash) {
                continue;
            }
            visited_lines.insert(hash);

            if !(&item.snippet).is_empty() {
                item.snippet.push_str(writer::LINE_SEPARATOR);
            }
            item.snippet.push_str(&line);
        }
    }

    if !should_break {
        let _ = write_cmd(&item, stdin, allowlist, denylist);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_line() {
        let (variable, command, command_options) =
            parse_variable_line("$ user : echo -e \"$(whoami)\\nroot\" --- --prevent-extra").unwrap();
        assert_eq!(command, " echo -e \"$(whoami)\\nroot\" ");
        assert_eq!(variable, "user");
        let opts = command_options.unwrap();
        assert_eq!(opts.header_lines, 0);
        assert_eq!(opts.column, None);
        assert_eq!(opts.delimiter, None);
        assert_eq!(opts.suggestion_type, SuggestionType::SingleSelection);
    }
}
