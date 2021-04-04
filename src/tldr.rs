use crate::display::Writer;
use crate::fetcher;
use crate::parser;
use crate::structures::cheat::VariableMap;
use anyhow::{Context, Error};
use regex::Regex;
use std::collections::HashSet;

use std::process::{self, Command, Stdio};

lazy_static! {
    pub static ref VAR_Tldr_REGEX: Regex = Regex::new(r"\{\{(.*?)\}\}").expect("Invalid regex");
    pub static ref NON_VAR_CHARS_REGEX: Regex = Regex::new(r"[^\da-zA-Z_]").expect("Invalid regex");
}

static VERSION_DISCLAIMER: &str = "The tldr client written in C (the default one in Homebrew) doesn't support markdown files, so navi can't use it.
The client written in Rust is recommended. The one available in npm works, too.";

fn convert_tldr_vars(line: &str) -> String {
    let caps = VAR_Tldr_REGEX.find_iter(&line);
    let mut new_line: String = line.to_string();
    for cap in caps {
        let braced_var = cap.as_str();
        let var = &braced_var[2..braced_var.len() - 2];
        let mut new_var = NON_VAR_CHARS_REGEX.replace_all(var, "_").to_string();
        if let Some(c) = new_var.chars().next() {
            if c.to_string().parse::<u8>().is_ok() {
                new_var = format!("example_{}", new_var);
            }
        }
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
        0,
        &mut variables,
        &mut visited_lines,
        writer,
        stdin,
    )?;
    Ok(Some(variables))
}

pub fn fetch(query: &str) -> Result<String, Error> {
    let args = [query, "--markdown"];

    let child = Command::new("tldr")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            eprintln!(
                "navi was unable to call tldr.
Make sure tldr is correctly installed.
Refer to https://github.com/tldr-pages/tldr for more info.

Note:
{}
",
                VERSION_DISCLAIMER
            );
            process::exit(34)
        }
    };

    let out = child.wait_with_output().context("Failed to wait for tldr")?;

    if let Some(0) = out.status.code() {
    } else {
        eprintln!(
            "Failed to call: 
tldr {}
 
Output:
{}

Error:
{}

Note:
Please make sure you're using a version that supports the --markdown flag.
If you are already using a supported version you can ignore this message. 
{}
",
            args.join(" "),
            String::from_utf8(out.stdout).unwrap_or_else(|_e| "Unable to get output message".to_string()),
            String::from_utf8(out.stderr).unwrap_or_else(|_e| "Unable to get error message".to_string()),
            VERSION_DISCLAIMER
        );
        process::exit(35)
    }

    let stdout = out.stdout;

    String::from_utf8(stdout).context("Output is invalid utf8")
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
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        writer: &mut dyn Writer,
        _files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>, Error> {
        let markdown = fetch(&self.query)?;
        read_all(&self.query, &markdown, stdin, writer)
    }
}
