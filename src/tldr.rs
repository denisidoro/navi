use crate::display::Writer;
use crate::fetcher::Fetcher;
use crate::parser;
use crate::structures::cheat::VariableMap;
use anyhow::Context;
use anyhow::Error;
use regex::Regex;
use std::collections::HashSet;
use std::process::{self, Command, Stdio};

lazy_static! {
    pub static ref VAR_TLDR_REGEX: Regex = Regex::new(r"\{\{(.*?)\}\}").expect("Invalid regex");
    pub static ref NON_VAR_CHARS_REGEX: Regex = Regex::new(r"[^\da-zA-Z_]").expect("Invalid regex");
}

fn convert_tldr_vars(line: &str) -> String {
    let caps = VAR_TLDR_REGEX.find_iter(&line);
    let mut new_line: String = line.to_string();
    for cap in caps {
        let braced_var = cap.as_str();
        let var = &braced_var[2..braced_var.len() - 2];
        let new_var = NON_VAR_CHARS_REGEX.replace_all(var, "_");
        let bracketed_var = format!("<{}>", new_var);
        new_line = new_line.replace(braced_var, &bracketed_var);
    }
    new_line
}

fn convert_tldr(line: &str) -> Result<String, Error> {
    let line = line.trim();
    let new_line = if line.starts_with('-') {
        format!("{}{}", "# ", &line[2..line.len() - 1])
    } else if line.starts_with('`') {
        convert_tldr_vars(&line[1..line.len() - 1])
    } else if line.starts_with('%') {
        line.to_string()
    } else {
        "".to_string()
    };
    Ok(new_line)
}

fn markdown_lines(query: &str, markdown: &str) -> impl Iterator<Item = Result<String, Error>> {
    format!(
        "% {}, tldr
    {}",
        query, markdown
    )
    .lines()
    .map(convert_tldr)
    .collect::<Vec<Result<String, Error>>>()
    .into_iter()
}

fn read_all(
    query: &str,
    markdown: &str,
    stdin: &mut std::process::ChildStdin,
    writer: &mut dyn Writer,
) -> Result<Option<VariableMap>, Error> {
    let mut variables = VariableMap::new();
    let mut visited_lines = HashSet::new();
    parser::read_lines(
        markdown_lines(query, markdown),
        "markdown",
        &mut variables,
        &mut visited_lines,
        writer,
        stdin,
    )?;
    Ok(Some(variables))
}

pub fn fetch(query: &str) -> Result<String, Error> {
    let child = Command::new("tldr")
        .args(&[query, "--markdown"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            eprintln!("navi was unable to call tldr");
            process::exit(33)
        }
    };

    let stdout = child
        .wait_with_output()
        .context("Failed to wait for tldr")?
        .stdout;

    String::from_utf8(stdout).context("Suggestions are invalid utf8")
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
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        writer: &mut dyn Writer,
    ) -> Result<Option<VariableMap>, Error> {
        eprintln!("TODO!!!!");
        let markdown = fetch(&self.query)?;
        read_all(&self.query, &markdown, stdin, writer)
    }
}
