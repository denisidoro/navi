use crate::cheat;
use crate::display;
use crate::filesystem;

use std::collections::HashMap;
use std::process;
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
        }
    }
}

pub fn call<F>(opts: Opts, stdin_fn: F) -> (String, Option<HashMap<String, cheat::Value>>)
where
    F: Fn(&mut process::ChildStdin) -> Option<HashMap<String, cheat::Value>>,
{
    let mut fzf_command = Command::new("fzf");

    fzf_command.args(&[
        "--preview-window",
        "up:2",
        "--with-nth",
        "1,2,3",
        "--delimiter",
        display::DELIMITER.to_string().as_str(),
        "--ansi",
        "--bind",
        "ctrl-j:down,ctrl-k:up",
        "--exact",
    ]);

    if opts.autoselect {
        fzf_command.arg("--select-1");
    }

    if opts.multi {
        fzf_command.arg("--multi");
    }

    if opts.copyable {
        fzf_command.args(&["--expect", "ctrl-y,enter"]);
    }

    if opts.preview {
        fzf_command.args(&[
            "--preview",
            format!("{} preview {{}}", filesystem::exe_string()).as_str(),
        ]);
    }

    if let Some(q) = opts.query {
        fzf_command.args(&["--query", &q]);
    }

    if let Some(f) = opts.filter {
        fzf_command.args(&["--filter", &f]);
    }

    if let Some(p) = opts.prompt {
        fzf_command.args(&["--prompt", &p]);
    }

    if opts.header_lines > 0 {
        fzf_command.args(&["--header-lines", format!("{}", opts.header_lines).as_str()]);
    }

    if let Some(o) = opts.overrides {
        o.as_str()
            .split(' ')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                fzf_command.arg(s);
            });
    }

    if !opts.suggestions {
        fzf_command.args(&["--print-query", "--no-select-1", "--height", "1"]);
    }

    let child = fzf_command
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

    let mut text = match out.status.code() {
        Some(0) | Some(1) => String::from_utf8(out.stdout).unwrap(),
        Some(130) => process::exit(130),
        _ => {
            let err = String::from_utf8(out.stderr)
                .unwrap_or_else(|_| "<stderr contains invalid UTF-8>".to_owned());
            panic!("External command failed:\n {}", err)
        }
    };
    text.truncate(text.len() - 1);

    (text, result)
}
