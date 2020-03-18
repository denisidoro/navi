use crate::display;
use crate::filesystem;
use crate::structures::fnv::HashLine;
use crate::structures::cheat::{VariableMap, SuggestionOpts, SuggestionType};
use crate::option::Config;
use crate::welcome;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::Write;

fn remove_quotes(txt: &str) -> String {
    txt.replace('"', "").replace('\'', "")
}

fn parse_opts(text: &str) -> SuggestionOpts {
    let mut header_lines: u8 = 0;
    let mut column: Option<u8> = None;
    let mut multi = false;
    let mut prevent_extra = false;
    let mut delimiter: Option<String> = None;

    let mut parts = text.split(' ');

    while let Some(p) = parts.next() {
        match p {
            "--multi" => multi = true,
            "--prevent-extra" => prevent_extra = true,
            "--header" | "--headers" | "--header-lines" => {
                header_lines = remove_quotes(parts.next().unwrap()).parse::<u8>().unwrap()
            }
            "--column" => {
                column = Some(remove_quotes(parts.next().unwrap()).parse::<u8>().unwrap())
            }
            "--delimiter" => delimiter = Some(remove_quotes(parts.next().unwrap()).to_string()),
            _ => (),
        }
    }

    SuggestionOpts {
        header_lines,
        column,
        delimiter,
        suggestion_type: match (multi, prevent_extra) {
            (true, _) => SuggestionType::MultipleSelections, // multi wins over allow-extra
            (false, false) => SuggestionType::SingleRecommendation,
            (false, true) => SuggestionType::SingleSelection,
        },
    }
}

fn parse_variable_line(line: &str) -> (&str, &str, Option<SuggestionOpts>) {
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
    stdin: &mut std::process::ChildStdin,
    set: &mut HashSet<u64>,
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
            let hash = line.hash_line();
            if set.contains(&hash) {
                continue;
            }
            set.insert(hash);

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
            // first snippet line
            else if (&snippet).is_empty() {
                snippet.push_str(&line);
            }
            // other snippet lines
            else {
                snippet.push_str(display::LINE_SEPARATOR);
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
    let paths = filesystem::cheat_paths(config);
    let folders = paths.split(':');
    let mut set = HashSet::new();

    for folder in folders {
        if let Ok(paths) = fs::read_dir(folder) {
            for path in paths {
                let path = path.unwrap().path();
                let path_str = path.to_str().unwrap();
                if path_str.ends_with(".cheat")
                    && read_file(path_str, &mut variables, stdin, &mut set)
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
            Some(SuggestionOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleRecommendation
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
        let mut set: HashSet<u64> = HashSet::new();
        read_file(path, &mut variables, child_stdin, &mut set);
        let mut result = VariableMap::new();
        let expected_suggestion = (
            r#" echo -e "$(whoami)\nroot" "#.to_string(),
            Some(SuggestionOpts {
                header_lines: 0,
                column: None,
                delimiter: None,
                suggestion_type: SuggestionType::SingleRecommendation,
            }),
        );
        let actual_suggestion = variables.get("ssh", "user");
        assert_eq!(expected_suggestion, actual_suggestion);
    }
}
