use crate::filesystem;
use ansi_term::Colour;
use std::env;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use regex::Regex;

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

fn parse_variable_line(line: &str) -> (&str, &str, &str) {
    let re = Regex::new(r"^\$\s*([^:]+):(.*)").unwrap();
    let caps = re.captures(line).unwrap();
    let variable = caps.get(1).unwrap().as_str().trim();
    let command = caps.get(2).unwrap().as_str();
    (variable, command, command)
}

fn read_file(path: &str, variables: &mut HashMap<String, String>, stdin: &mut std::process::ChildStdin) {
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
                let (variable, command, _) = parse_variable_line(&line[..]);
                variables.insert(format!("{};{}", tags, variable), String::from(command));
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

pub fn read_all(stdin: &mut std::process::ChildStdin) -> HashMap<String, String> {
    let mut variables: HashMap<String, String> = HashMap::new();

    let folders_str =
        env::var("NAVI_PATH").unwrap_or(format!("{}/../../cheats", filesystem::exe_path_string()));
    let folders = folders_str.split(':');

    for folder in folders {
        let paths = fs::read_dir(folder).unwrap();
        for path in paths {
            read_file(
                path.unwrap().path().into_os_string().to_str().unwrap(),
                &mut variables,
                stdin,
            );
        }
    }

    variables
}
