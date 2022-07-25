use crate::parser::Parser;
use crate::prelude::*;
use crate::structures::cheat::VariableMap;
use crate::structures::fetcher;
use std::io::Write;
use std::process::{self, Command};

fn map_line(line: &str) -> String {
    line.trim().trim_end_matches(':').to_string()
}

fn lines(query: &str, markdown: &str) -> impl Iterator<Item = Result<String>> {
    format!(
        "% {}, cheat.sh
{}",
        query, markdown
    )
    .lines()
    .map(|line| Ok(map_line(line)))
    .collect::<Vec<Result<String>>>()
    .into_iter()
}

fn read_all(query: &str, cheat: &str, writer: &mut dyn Write) -> Result<Option<VariableMap>> {
    let mut parser = Parser::new(writer, true);

    if cheat.starts_with("Unknown topic.") {
        eprintln!(
            "`{}` not found in cheatsh.

Output:
{}
",
            query, cheat
        );
        process::exit(35)
    }

    parser.read_lines(lines(query, cheat), "cheat.sh", None)?;

    Ok(Some(parser.variables))
}

pub fn fetch(query: &str) -> Result<String> {
    let args = ["-qO-", &format!("cheat.sh/{}", query)];

    let child = Command::new("wget")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            eprintln!(
                "navi was unable to call wget.
Make sure wget is correctly installed."
            );
            process::exit(34)
        }
    };

    let out = child.wait_with_output().context("Failed to wait for wget")?;

    if let Some(0) = out.status.code() {
    } else {
        eprintln!(
            "Failed to call:
wget {}

Output:
{}

Error:
{}
",
            args.join(" "),
            String::from_utf8(out.stdout).unwrap_or_else(|_e| "Unable to get output message".to_string()),
            String::from_utf8(out.stderr).unwrap_or_else(|_e| "Unable to get error message".to_string())
        );
        process::exit(35)
    }

    let stdout = out.stdout;
    let plain_bytes = strip_ansi_escapes::strip(&stdout)?;

    String::from_utf8(plain_bytes).context("Output is invalid utf8")
}

pub struct Fetcher {
    query: String,
}

impl Fetcher {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}

impl fetcher::Fetcher for Fetcher {
    fn fetch(&self, writer: &mut dyn Write, _files: &mut Vec<String>) -> Result<Option<VariableMap>> {
        let cheat = fetch(&self.query)?;
        read_all(&self.query, &cheat, writer)
    }
}
