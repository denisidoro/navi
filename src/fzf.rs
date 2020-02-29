use crate::filesystem;

use std::collections::HashMap;
use std::process;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct Opts {
    pub query: Option<String>,
    pub filter: Option<String>,
    pub autoselect: bool,
}

pub fn call<F>(opts: Opts, stdin_fn: F) -> (process::Output, HashMap<String, String>)
where
    F: Fn(&mut process::ChildStdin) -> HashMap<String, String>,
{
    let mut c = Command::new("fzf");

    c.args(&[
        "--height",
        "100%",
        "--preview-window",
        "up:2",
        "--with-nth",
        "1,2,3",
        "--delimiter",
        "\t",
        "--ansi",
        "--preview",
        format!("{} preview {{}}", filesystem::exe_string()).as_str(),
    ]);

    if let Some(q) = opts.query {
        c.args(&["--query", &q]);
    }

    if let Some(f) = opts.filter {
        c.args(&["--filter", &f]);
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
