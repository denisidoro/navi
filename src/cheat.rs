use crate::display;
use crate::filesystem;
use crate::option::Config;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct SuggestionOpts {
    pub header_lines: u8,
    pub column: Option<u8>,
    pub delimiter: Option<String>,
    pub suggestion_type: SuggestionType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuggestionType {
    /// fzf will not print any suggestions.
    Disabled,
    /// fzf will only select one of the suggestions
    SingleSelection,
    /// fzf will select multiple ones of the suggestions
    MultipleSelections,
    /// fzf will select one of the suggestions or use the Query
    SingleRecommendation,
    /// initial snippet selection
    SnippetSelection,
}

pub type Suggestion = (String, Option<SuggestionOpts>);

fn remove_quote(txt: &str) -> String {
    txt.replace('"', "").replace('\'', "")
}

fn parse_opts(text: &str) -> SuggestionOpts {
    let mut header_lines: u8 = 0;
    let mut column: Option<u8> = None;
    let mut multi = false;
    let mut allow_extra = false;
    let mut delimiter: Option<String> = None;

    let mut parts = text.split(' ');

    while let Some(p) = parts.next() {
        match p {
            "--multi" => multi = true,
            "--allow-extra" => allow_extra = true,
            "--header" | "--headers" | "--header-lines" => {
                header_lines = remove_quote(parts.next().unwrap()).parse::<u8>().unwrap()
            }
            "--column" => column = Some(remove_quote(parts.next().unwrap()).parse::<u8>().unwrap()),
            "--delimiter" => delimiter = Some(remove_quote(parts.next().unwrap()).to_string()),
            _ => (),
        }
    }

    SuggestionOpts {
        header_lines,
        column,
        delimiter,
        suggestion_type: match (multi, allow_extra) {
            (true, _) => SuggestionType::MultipleSelections, // multi wins over allow-extra
            (false, true) => SuggestionType::SingleRecommendation,
            (false, false) => SuggestionType::SingleSelection,
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
    } else if let Ok(_) = stdin.write_all(
        display::format_line(
            &tags[..],
            &comment[..],
            &snippet[3..],
            tag_width,
            comment_width,
        )
        .as_bytes(),
    ) {
        true
    } else {
        false
    }
}

fn read_file(
    path: &str,
    variables: &mut HashMap<String, Suggestion>,
    stdin: &mut std::process::ChildStdin,
) {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");

    let (tag_width, comment_width) = *display::WIDTHS;

    if let Ok(lines) = filesystem::read_lines(path) {
        for l in lines {
            let line = l.unwrap();

            // tag
            if line.starts_with('%') {
                if !write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin) {
                    break;
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
                    break;
                }
                snippet = String::from("");
                comment = String::from(&line[2..]);
            }
            // variable
            else if line.starts_with('$') {
                if !write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin) {
                    break;
                }
                snippet = String::from("");
                let (variable, command, opts) = parse_variable_line(&line);
                variables.insert(
                    format!("{};{}", tags, variable),
                    (String::from(command), opts),
                );
            }
            // blank
            else if line.is_empty() {
            }
            // snippet
            else {
                snippet.push_str(display::LINE_SEPARATOR);
                snippet.push_str(&line);
            }
        }
    }

    write_cmd(&tags, &comment, &snippet, tag_width, comment_width, stdin);
}

pub fn read_all(
    config: &Config,
    stdin: &mut std::process::ChildStdin,
) -> HashMap<String, Suggestion> {
    let mut variables: HashMap<String, Suggestion> = HashMap::new();

    let mut fallback: String = String::from("");
    let folders_str = config.path.as_ref().unwrap_or_else(|| {
        if let Some(f) = filesystem::cheat_pathbuf() {
            fallback = filesystem::pathbuf_to_string(f);
        }
        &fallback
    });
    let folders = folders_str.split(':');

    for folder in folders {
        if let Ok(paths) = fs::read_dir(folder) {
            for path in paths {
                let path_os_str = path.unwrap().path().into_os_string();
                let path_str = path_os_str.to_str().unwrap();
                if path_str.ends_with(".cheat") {
                    read_file(path_str, &mut variables, stdin);
                }
            }
        }
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
        let mut variables: HashMap<String, Suggestion> = HashMap::new();
        let mut child = Command::new("cat").stdin(Stdio::piped()).spawn().unwrap();
        let child_stdin = child.stdin.as_mut().unwrap();
        read_file(path, &mut variables, child_stdin);
        let mut result: HashMap<String, (String, std::option::Option<_>)> = HashMap::new();
        result.insert(
            "ssh;user".to_string(),
            (
                r#" echo -e "$(whoami)\nroot" "#.to_string(),
                Some(SuggestionOpts {
                    header_lines: 0,
                    column: None,
                    delimiter: None,
                    suggestion_type: SuggestionType::SingleRecommendation,
                }),
            ),
        );
        assert_eq!(variables, result);
    }
}
