use crate::filesystem;
use crate::cheat;

use std::collections::HashMap;
use std::process;
use std::process::{Command, Stdio};

pub struct Opts<'a> {
    pub query: Option<String>,
    pub filter: Option<String>,
    pub preview: bool, 
    pub autoselect: bool,
    pub overrides: Option<&'a String>,
    pub header_lines: u8,
    pub multi: bool
}

impl Default for Opts<'_> {
    fn default() -> Self { 
        Self {
            query: None,
        filter: None,
        preview: true,
        autoselect: true,
        overrides: None,
        header_lines: 1,
        multi: false
    }
}
}

pub fn call<F>(opts: Opts, stdin_fn: F) -> (process::Output, HashMap<String, cheat::Value>)
where
    F: Fn(&mut process::ChildStdin) -> HashMap<String, cheat::Value>,
{
    let mut c = Command::new("fzf");

    c.args(&[
        "--preview-window",
        "up:2",
        "--with-nth",
        "1,2,3",
        "--delimiter",
        "\t",
        "--ansi",
        "--bind",
        "ctrl-j:down,ctrl-k:up",
        "--expect",
        "ctrl-y"
    ]);

    if opts.autoselect {
        c.arg("--select-1");
    }

    if opts.multi {
        c.arg("--multi");
    }

    if opts.preview {
        c.args(&["--preview", format!("{} preview {{}}", filesystem::exe_string()).as_str()]);
    }
        
    if let Some(q) = opts.query {
        c.args(&["--query", &q]);
    }

    if let Some(f) = opts.filter {
        c.args(&["--filter", &f]);
    }

    if opts.header_lines > 0 {
        c.args(&["--header-lines", format!("{}", opts.header_lines).as_str()]);
    }

    if let Some(o) = opts.overrides {
        o.as_str()
        .split(' ')
        .map(|s| s.to_string())
        .filter(|s| s.len() > 0)
        .for_each(|s| {
            c.arg(s);
        });
    }

    let mut child = c
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let stdin = child
        .stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")
        .unwrap();

    let result = stdin_fn(stdin);

    (child.wait_with_output().unwrap(), result)
}
