use crate::display::Writer;
use crate::fetcher::Fetcher;
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
        "% {}, tldr
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
    parser::read_lines(lines(query, cheat), "cheatsh", &mut variables, &mut visited_lines, writer, stdin)?;
    Ok(Some(variables))
}

pub fn fetch(query: &str) -> Result<String, Error> {
    let child = Command::new("wget")
        .args(&["-qO-", &format!("cheat.sh/{}", query)])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            eprintln!("navi was unable to call curl");
            process::exit(33)
        }
    };

    let stdout = child.wait_with_output().context("Failed to wait for curl")?.stdout;

    let plain_bytes = strip_ansi_escapes::strip(&stdout)?;

    String::from_utf8(plain_bytes).context("Suggestions are invalid utf8")
}

pub struct Foo {
    query: String,
}

impl Foo {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}

impl Fetcher for Foo {
    fn fetch(&self, stdin: &mut std::process::ChildStdin, writer: &mut dyn Writer) -> Result<Option<VariableMap>, Error> {
        eprintln!("TODO!!!!");
        let cheat = fetch(&self.query)?;
        read_all(&self.query, &cheat, stdin, writer)
    }
}
