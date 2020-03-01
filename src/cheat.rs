use crate::filesystem;
use crate::option::Config;

use ansi_term::Colour;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Default)]
pub struct SuggestionOpts {
    pub header_lines: u8,
    pub column: Option<u8>,
    pub multi: bool
}

pub type Value = (String, Option<SuggestionOpts>);

fn gen_snippet(snippet: &str, line: &str) -> String {
    if snippet.is_empty() {
        line.to_string()
    } else {
        format!("{}{}", &snippet[..snippet.len() - 2], line)
    }
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", &text[..length - 1])
    } else {
        format!("{:width$}", text, width = length)
    }
}

fn parse_opts(text: &str) -> SuggestionOpts {
    SuggestionOpts {
        header_lines: 0,
        column: None,
    }
}

fn parse_variable_line(line: &str) -> (&str, &str, Option<SuggestionOpts>) {
    let re = Regex::new(r"^\$\s*([^:]+):(.*)").unwrap();
    let caps = re.captures(line).unwrap();
    let variable = caps.get(1).unwrap().as_str().trim();
    let mut command_plus_opts = caps.get(2).unwrap().as_str().split("---");
    let command = command_plus_opts.next().unwrap();
    let opts = match command_plus_opts.next() {
        Some(o) => Some(parse_opts(o)),
        None => None,
    };
    (variable, command, opts)
}

fn read_file(
    path: &str,
    variables: &mut HashMap<String, Value>,
    stdin: &mut std::process::ChildStdin,
) {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");

    if let Ok(lines) = filesystem::read_lines(path) {
        for l in lines {
            let line = l.unwrap();
            if line.starts_with('%') {
                tags = String::from(&line[2..]);
            } else if line.starts_with('#') {
                comment = String::from(&line[2..]);
            } else if line.starts_with('$') {
                let (variable, command, opts) = parse_variable_line(&line[..]);
                variables.insert(format!("{};{}", tags, variable), (String::from(command), opts));
            }
            // TODO
            else if line.ends_with('\\') {
                snippet = if !snippet.is_empty() {
                    format!("{}{}", &snippet[..snippet.len() - 2], line)
                } else {
                    line
                }
            } else if line.is_empty() {
            } else {
                let full_snippet = gen_snippet(&snippet, &line);
                match stdin.write(
                    format!(
                        "{col0}\t{col1}\t{col2}\t{tags}\t{comment}\t{snippet}\t\n",
                        col0 = Colour::Red.paint(limit_str(&tags[..], 16)),
                        col1 = Colour::Blue.paint(limit_str(&comment[..], 26)),
                        col2 = &full_snippet,
                        tags = tags,
                        comment = comment,
                        snippet = &full_snippet
                    )
                    .as_bytes(),
                ) {
                    Ok(_) => snippet = String::from(""),
                    Err(_) => break,
                }
            }
        }
    }
}

pub fn read_all(config: &Config, stdin: &mut std::process::ChildStdin) -> HashMap<String, Value> {
    let mut variables: HashMap<String, Value> = HashMap::new();

    let fallback = format!("{}/cheats", filesystem::exe_path_string());
    let folders_str = config.path.as_ref().unwrap_or(&fallback);
    let folders = folders_str.split(':');

    for folder in folders {
        if let Ok(paths) = fs::read_dir(folder) {
            for path in paths {
                read_file(
                    path.unwrap().path().into_os_string().to_str().unwrap(),
                    &mut variables,
                    stdin,
                );
            }
        }
    }

    variables
}
