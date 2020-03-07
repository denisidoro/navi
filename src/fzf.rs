use crate::cheat;
use crate::filesystem;

use std::collections::HashMap;
use std::fs;
use std::process;
use std::io::{Read,Write};
use std::fs::OpenOptions;
use std::process::{Command, Stdio};

pub struct Opts<'a> {
    pub query: Option<String>,
    pub filter: Option<String>,
    pub prompt: Option<String>,
    pub preview: bool,
    pub autoselect: bool,
    pub overrides: Option<&'a String>, // TODO: remove &'a
    pub header_lines: u8,
    pub multi: bool,
    pub copyable: bool,
    pub suggestions: bool,
    pub nth: Option<u8>,
}

impl Default for Opts<'_> {
    fn default() -> Self {
        Self {
            query: None,
            filter: None,
            preview: true,
            autoselect: true,
            overrides: None,
            header_lines: 0,
            prompt: None,
            multi: false,
            copyable: false,
            suggestions: true,
            nth: None,
        }
    }
}

pub fn call<F>(opts: Opts, stdin_fn: F) -> (String, Option<HashMap<String, cheat::Value>>)
where
    F: Fn(&mut process::ChildStdin) -> Option<HashMap<String, cheat::Value>>,
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
        "--exact",
    ]);

    if opts.autoselect {
        c.arg("--select-1");
    }

    if opts.multi {
        c.arg("--multi");
    }

    if opts.copyable {
        c.args(&["--expect", "ctrl-y,enter"]);
    }

    if opts.preview {
        c.args(&[
            "--preview",
            format!("{} preview {{}}", filesystem::exe_string()).as_str(),
        ]);
    }

    if let Some(q) = opts.query {
        c.args(&["--query", &q]);
    }

    if let Some(f) = opts.filter {
        c.args(&["--filter", &f]);
    }

    if let Some(p) = opts.prompt {
        c.args(&["--prompt", &p]);
    }

    if let Some(n) = opts.nth {
        c.args(&["--nth", &n.to_string()]);
    }

    if opts.header_lines > 0 {
        c.args(&["--header-lines", format!("{}", opts.header_lines).as_str()]);
    }

    if let Some(o) = opts.overrides {
        o.as_str()
            .split(' ')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                c.arg(s);
            });
    }

    if !opts.suggestions {
        c.args(&["--print-query", "--no-select-1", "--height", "1"]);
    }

   /*let tty = OpenOptions::new()
                .read(true)
                .write(true)
                .append(true)
                .open("/dev/tty")
                .unwrap();*/

    /*let child = c
        .stdin(Stdio::piped())
        .stdout(tty.try_clone().unwrap())
        .stderr(tty.try_clone().unwrap())
        .spawn();*/

    let child = c
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn();

    let mut child = match child {
        Ok(x) => x,
        Err(_) => {
            eprintln!("navi was unable to call fzf.\nPlease make sure it's correctly installed\nRefer to https://github.com/junegunn/fzf for more info.");
            process::exit(33)
        }
    };

    let stdin = child.stdin.as_mut().unwrap();
    let result = stdin_fn(stdin);

    let out = child.wait_with_output().unwrap();
    (String::from_utf8(out.stdout).unwrap(), result)
}
