use crate::display::Writer;
use crate::fetcher;
use crate::parser;
use crate::structures::cheat::VariableMap;
use anyhow::Context;
use anyhow::Error;
use std::collections::HashSet;
use std::process::{self, Command, Stdio};

fn map_line(line: &str) -> Result<String, Error> {
    let line = line.trim().trim_end_matches(':');
    Ok(line.to_string())
}

fn lines(query: &str, markdown: &str) -> impl Iterator<Item = Result<String, Error>> {
    format!(
        "% {}, cheat.sh
{}",
        query, markdown
    )
    .lines()
    .map(map_line)
    .collect::<Vec<Result<String, Error>>>()
    .into_iter()
}

fn read_all(query: &str, cheat: &str, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error> {
    let mut variables = VariableMap::new();
    let mut visited_lines = HashSet::new();
    parser::read_lines(lines(query, cheat), "cheat.sh", &mut variables, &mut visited_lines, writer, stdin)?;
    Ok(Some(variables))
}

pub fn fetch(query: &str) -> Result<String, Error> {
    let args = ["-qO-", &format!("cheat.sh/{}", query)];

    let child = Command::new("wget").args(&args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            eprintln!(
                "navi was unable to call wget.
Make sure wget is correctly installed.");
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
    fn fetch(&self, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error> {
        let cheat = fetch(&self.query)?;
        read_all(&self.query, &cheat, stdin, writer)
    }
}
