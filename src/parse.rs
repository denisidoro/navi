use crate::filesystem;
use ansi_term::Colour;
use std::fs;
use std::io::Write;

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

fn read_file(path: &str, stdin: &mut std::process::ChildStdin) {
    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");

    if let Ok(lines) = filesystem::read_lines(path) {
        for l in lines {
            let line = l.unwrap();
            if line.starts_with('%') {
                tags = line;
            } else if line.starts_with('#') {
                comment = line;
            } else if line.starts_with('$') {
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
                        col0 = Colour::Red.paint(limit_str(&tags[2..], 16)),
                        col1 = Colour::Blue.paint(limit_str(&comment[2..], 26)),
                        col2 = &full_snippet,
                        tags = &tags[2..],
                        comment = &comment[2..],
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

pub fn read_all(stdin: &mut std::process::ChildStdin) {
    let paths = fs::read_dir("./cheats").unwrap();
    for path in paths {
        read_file(
            path.unwrap().path().into_os_string().to_str().unwrap(),
            stdin,
        );
    }
}
