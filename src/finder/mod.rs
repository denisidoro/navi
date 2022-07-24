use crate::prelude::*;
use crate::structures::cheat::VariableMap;
use crate::writer;
use std::io::Write;
use std::process::{self, Output};
use std::process::{Command, Stdio};
pub mod structures;
pub use post::process;
use structures::Opts;
use structures::SuggestionType;

mod post;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum FinderChoice {
    Fzf,
    Skim,
}

impl FromStr for FinderChoice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fzf" => Ok(FinderChoice::Fzf),
            "skim" => Ok(FinderChoice::Skim),
            _ => Err("no match"),
        }
    }
}

pub trait Finder {
    fn call<F>(&self, opts: Opts, stdin_fn: F) -> Result<(String, Option<VariableMap>, Vec<String>)>
    where
        F: Fn(&mut Box<&mut dyn Write>, &mut Vec<String>) -> Result<Option<VariableMap>>;
}

fn parse(out: Output, opts: Opts) -> Result<String> {
    let text = match out.status.code() {
        Some(0) | Some(1) | Some(2) => {
            String::from_utf8(out.stdout).context("Invalid utf8 received from finder")?
        }
        Some(130) => process::exit(130),
        _ => {
            let err = String::from_utf8(out.stderr)
                .unwrap_or_else(|_| "<stderr contains invalid UTF-8>".to_owned());
            panic!("External command failed:\n {}", err)
        }
    };

    let output = post::parse_output_single(text, opts.suggestion_type)?;
    post::process(output, opts.column, opts.delimiter.as_deref(), opts.map)
}

impl Finder for FinderChoice {
    fn call<F>(&self, finder_opts: Opts, stdin_fn: F) -> Result<(String, Option<VariableMap>, Vec<String>)>
    where
        F: Fn(&mut Box<&mut dyn Write>, &mut Vec<String>) -> Result<Option<VariableMap>>,
    {
        let finder_str = match self {
            Self::Fzf => "fzf",
            Self::Skim => "sk",
        };

        let mut command = Command::new(&finder_str);
        let opts = finder_opts.clone();

        let preview_height = match self {
            FinderChoice::Skim => 3,
            _ => 2,
        };

        let bindings = if opts.suggestion_type == SuggestionType::MultipleSelections {
            ",ctrl-r:toggle-all"
        } else {
            ""
        };

        command.args(&[
            "--preview",
            "",
            "--preview-window",
            format!("up:{}:nohidden", preview_height).as_str(),
            "--with-nth",
            "1,2,3",
            "--delimiter",
            writer::DELIMITER.to_string().as_str(),
            "--ansi",
            "--bind",
            format!("ctrl-j:down,ctrl-k:up{}", bindings).as_str(),
            "--exact",
        ]);

        if !opts.prevent_select1 {
            if let Self::Fzf = self {
                command.arg("--select-1");
            }
        }

        match opts.suggestion_type {
            SuggestionType::MultipleSelections => {
                command.arg("--multi");
            }
            SuggestionType::Disabled => {
                if let Self::Fzf = self {
                    command.args(&["--print-query", "--no-select-1"]);
                };
            }
            SuggestionType::SnippetSelection => {
                command.args(&["--expect", "ctrl-y,ctrl-o,enter"]);
            }
            SuggestionType::SingleRecommendation => {
                command.args(&["--print-query", "--expect", "tab,enter"]);
            }
            _ => {}
        }

        if let Some(p) = opts.preview {
            command.args(&["--preview", &p]);
        }

        if let Some(q) = opts.query {
            command.args(&["--query", &q]);
        }

        if let Some(f) = opts.filter {
            command.args(&["--filter", &f]);
        }

        if let Some(d) = opts.delimiter {
            command.args(&["--delimiter", &d]);
        }

        if let Some(h) = opts.header {
            command.args(&["--header", &h]);
        }

        if let Some(p) = opts.prompt {
            command.args(&["--prompt", &p]);
        }

        if let Some(pw) = opts.preview_window {
            command.args(&["--preview-window", &pw]);
        }

        if opts.header_lines > 0 {
            command.args(&["--header-lines", format!("{}", opts.header_lines).as_str()]);
        }

        if let Some(o) = opts.overrides {
            o.as_str()
                .split(' ')
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .for_each(|s| {
                    command.arg(s);
                });
        }

        let child = command
            .env("SHELL", CONFIG.finder_shell())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(x) => x,
            Err(_) => {
                let repo = match self {
                    Self::Fzf => "https://github.com/junegunn/fzf",
                    Self::Skim => "https://github.com/lotabout/skim",
                };
                eprintln!(
                    "navi was unable to call {cmd}.
                Please make sure it's correctly installed.
                Refer to {repo} for more info.",
                    cmd = &finder_str,
                    repo = repo
                );
                process::exit(33)
            }
        };

        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("Unable to acquire stdin of finder"))?;
        let mut writer: Box<&mut dyn Write> = Box::new(stdin);

        let mut files = vec![];

        let result_map = stdin_fn(&mut writer, &mut files).context("Failed to pass data to finder")?;

        let out = child.wait_with_output().context("Failed to wait for finder")?;

        let output = parse(out, finder_opts).context("Unable to get output")?;
        Ok((output, result_map, files))
    }
}
