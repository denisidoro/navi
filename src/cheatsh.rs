use crate::parser;
use crate::structures::cheat::VariableMap;
use crate::structures::fetcher;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashSet;
use std::process::{Command, Stdio};

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

fn read_all(query: &str, cheat: &str, stdin: &mut std::process::ChildStdin) -> Result<Option<VariableMap>> {
    let mut variables = VariableMap::new();
    let mut visited_lines = HashSet::new();

    parser::read_lines(
        lines(query, cheat),
        "cheat.sh",
        0,
        &mut variables,
        &mut visited_lines,
        stdin,
        None,
        None,
    )?;
    Ok(Some(variables))
}

pub fn fetch(query: &str) -> Result<String> {
    let args = ["-qO-", &format!("cheat.sh/{}?T", query)];

    let child = Command::new("wget")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            let msg = "navi was unable to call wget.
Make sure wget is correctly installed.";
            eprintln!("{}", msg.replace('\n', "\r\n"));
            return Err(anyhow!("failed"));
        }
    };

    let out = child.wait_with_output().context("Failed to wait for wget")?;

    if let Some(0) = out.status.code() {
    } else {
        let msg = format!(
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
        eprintln!("{}", msg.replace('\n', "\r\n"));
        return Err(anyhow!("failed"));
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

pub const ERROR_MSG: &str = "# Terminate navi

exit 1";

impl fetcher::Fetcher for Fetcher {
    fn fetch(
        &self,
        stdin: &mut std::process::ChildStdin,
        _files: &mut Vec<String>,
    ) -> Result<Option<VariableMap>> {
        let mut cheat = fetch(&self.query).unwrap_or_else(|_| ERROR_MSG.into());

        if cheat.starts_with("Unknown topic.") {
            let msg = format!(
                "`{}` not found in cheatsh.

Output:
{} ",
                &self.query, cheat
            );
            eprintln!("{}", msg.replace('\n', " "));
            cheat = ERROR_MSG.into();
        }

        read_all(&self.query, &cheat, stdin)
    }
}
