use crate::display;
use crate::filesystem;
use crate::structures::cheat::VariableMap;
use crate::structures::fnv::HashLine;
use crate::structures::fzf::{Opts as FzfOpts, SuggestionType};
use crate::structures::option::Config;
use crate::welcome;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::Write;

fn parse_opts(text: &str) -> FzfOpts {
    let mut multi = false;
    let mut prevent_extra = false;
    let mut opts = FzfOpts::default();
    let parts_vec = shellwords::split(text).unwrap();
    let mut parts = parts_vec.into_iter();

    while let Some(p) = parts.next() {
        match p.as_str() {
            "--multi" => multi = true,
            "--prevent-extra" => prevent_extra = true,
            "--headers" | "--header-lines" => {
                opts.header_lines = parts.next().unwrap().parse::<u8>().unwrap()
            }
            "--column" => opts.column = Some(parts.next().unwrap().parse::<u8>().unwrap()),
            "--delimiter" => opts.delimiter = Some(parts.next().unwrap().to_string()),
            "--query" => opts.query = Some(parts.next().unwrap().to_string()),
            "--filter" => opts.filter = Some(parts.next().unwrap().to_string()),
            "--preview" => opts.preview = Some(parts.next().unwrap().to_string()),
            "--preview-window" => opts.preview_window = Some(parts.next().unwrap().to_string()),
            "--header" => opts.header = Some(parts.next().unwrap().to_string()),
            "--overrides" => opts.overrides = Some(parts.next().unwrap().to_string()), 
            _ => (),
        }
    }

    let suggestion_type = match (multi, prevent_extra) {
        (true, _) => SuggestionType::MultipleSelections, // multi wins over allow-extra
        (false, false) => SuggestionType::SingleRecommendation,
        (false, true) => SuggestionType::SingleSelection,
    };
    opts.suggestion_type = suggestion_type;

    opts
}

fn parse_variable_line(line: &str) -> (&str, &str, Option<FzfOpts>) {
    let re = Regex::new(r"^\$\s*([^:]+):(.*)").unwrap();
    let caps = re.captures(line).unwrap();
    let variable = caps.get(1).unwrap().as_str().trim();
    let mut command_plus_opts = caps.get(2).unwrap().as_str().split("---");
    let command = command_plus_opts.next().unwrap();
    let command_options = command_plus_opts.next().map(parse_opts);
    (variable, command, command_options)
}

fn write_cmd(
    tags: &str,
    comment: &str,
    snippet: &str,
    tag_width: usize,
    comment_width: usize,
    stdin: &mut std::process::ChildStdin,
) -> bool {
    if snippet.is_empty() {
        true
    } else {
        stdin
            .write_all(
                display::format_line(&tags, &comment, &snippet, tag_width, comment_width)
                    .as_bytes(),
            )
            .is_ok()
    }
}

fn read_file(
    path: &str,
    variables: &mut VariableMap,
    visited_lines: &mut HashSet<u64>,
    stdin: &mut std::process::ChildStdin,
) -> bool {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");
    let mut should_break = false;

    let (tag_width, comment_width) = *display::WIDTHS;

    if let Ok(lines) = filesystem::read_lines(path) {
        for l in lines {
            if should_break {
                break;
            }

            let line = l.unwrap();

            // duplicate
            if !tags.is_empty() && !comment.is_empty() {}

            // blank
            if line.is_empty() {
            }
            // tag
            else if line.starts_with('%') {
                if !write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin) {
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
                if !write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin) {
                    should_break = true
                }
                snippet = String::from("");
                comment = String::from(&line[2..]);
            }
            // variable
            else if line.starts_with('$') {
                if !write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin) {
                    should_break = true
                }
                snippet = String::from("");
                let (variable, command, opts) = parse_variable_line(&line);
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
            write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin);
        }

        return true;
    }

    false
}

pub fn read_all(config: &Config, stdin: &mut std::process::ChildStdin) -> VariableMap {
    let mut variables = VariableMap::new();
    let mut found_something = false;
    let mut visited_lines = HashSet::new();
    let paths = filesystem::cheat_paths(config);
    let folders = paths.split(':');

    for folder in folders {
        if let Ok(paths) = fs::read_dir(folder) {
            for path in paths {
                let path = path.unwrap().path();
                let path_str = path.to_str().unwrap();
                if path_str.ends_with(".cheat")
                    && read_file(path_str, &mut variables, &mut visited_lines, stdin)
                    && !found_something
                {
                    found_something = true;
                }
            }
        }
    }

    if !found_something {
        welcome::cheatsheet(stdin);
    }

    variables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable_line() {
        let (variable, command, command_options) =
            parse_variable_line("$ user : echo -e \"$(whoami)\\nroot\" --- --allow-extra");
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
        read_file(path, &mut variables, &mut visited_lines, child_stdin);
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
}
