use crate::display;
use crate::filesystem;
use crate::option::Config;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub struct SuggestionOpts {
    pub header_lines: u8,
    pub column: Option<u8>,
    pub multi: bool,
    pub delimiter: Option<String>,
}

pub type Value = (String, Option<SuggestionOpts>);

fn gen_snippet(snippet: &str, line: &str) -> String {
    if snippet.is_empty() {
        line.to_string()
    } else {
        format!("{}{}", &snippet[..snippet.len() - 2], line)
    }
}

fn remove_quote(txt: &str) -> String {
    txt.replace('"', "").replace('\'', "")
}

fn parse_opts(text: &str) -> SuggestionOpts {
    let mut header_lines: u8 = 0;
    let mut column: Option<u8> = None;
    let mut multi = false;
    let mut delimiter: Option<String> = None;

    let mut parts = text.split(' ');

    while let Some(p) = parts.next() {
        match p {
            "--multi" => multi = true,
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
        multi,
        delimiter,
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
) -> bool {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");

    let (tag_width, comment_width) = *display::WIDTHS;

    if let Ok(lines) = filesystem::read_lines(path) {
        for l in lines {
            let line = l.unwrap();

            // tag
            if line.starts_with('%') {
                tags = String::from(&line[2..]);
            }
            // metacomment
            else if line.starts_with(';') {
            }
            // comment
            else if line.starts_with('#') {
                comment = String::from(&line[2..]);
            }
            // variable
            else if line.starts_with('$') {
                let (variable, command, opts) = parse_variable_line(&line[..]);
                variables.insert(
                    format!("{};{}", tags, variable),
                    (String::from(command), opts),
                );
            }
            // snippet with line break
            else if line.ends_with('\\') {
                snippet = if !snippet.is_empty() {
                    format!("{}{}", &snippet[..snippet.len() - 2], line)
                } else {
                    line
                }
            }
            // blank
            else if line.is_empty() {
            }
            // snippet
            else {
                let full_snippet = gen_snippet(&snippet, &line);
                match stdin.write_all(
                    display::format_line(
                        &tags[..],
                        &comment[..],
                        &full_snippet[..],
                        tag_width,
                        comment_width,
                    )
                    .as_bytes(),
                ) {
                    Ok(_) => snippet = String::from(""),
                    Err(_) => break,
                }
            }
        }

        return true;
    }

    false
}

fn add_msg(
    tag: &str,
    comment: &str,
    snippet: &str,
    stdin: &mut std::process::ChildStdin,
) {
   stdin.write_all(display::format_line(tag, comment, snippet, 25, 60).as_bytes() ).unwrap();
}

pub fn read_all(config: &Config, stdin: &mut std::process::ChildStdin) -> HashMap<String, Value> {
    let mut variables: HashMap<String, Value> = HashMap::new();
    let mut found_something = false;

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
                if path_str.ends_with(".cheat") && read_file(path_str, &mut variables, stdin) {
                    found_something = true;
                }
            }
        }
    }

    if !found_something {
        add_msg("Cheatsheets", "Download default cheatsheets", "navi repo add default", stdin);
        add_msg("Cheatsheets", "Browse for cheatsheets to install", "navi repo browse", stdin);
        add_msg("Shell", "Install shell widget", "navi shell install", stdin);
    }

    variables
}
