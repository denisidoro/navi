use super::{get_column, parse_output_single, Finder};
use crate::display;
use crate::structures::cheat::VariableMap;
use crate::structures::finder::{Opts, SuggestionType};
use anyhow::Context;
use anyhow::Error;
use std::process;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct FzfFinder;

impl Finder for FzfFinder {
    fn call<F>(&self, opts: Opts, stdin_fn: F) -> Result<(String, Option<VariableMap>), Error>
    where
        F: Fn(&mut process::ChildStdin) -> Result<Option<VariableMap>, Error>,
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

        match opts.suggestion_type {
            SuggestionType::MultipleSelections => {
                fzf_command.arg("--multi");
            }
            SuggestionType::Disabled => {
                fzf_command.args(&["--print-query", "--no-select-1", "--height", "1"]);
            }
            SuggestionType::SnippetSelection => {
                fzf_command.args(&["--expect", "ctrl-y,enter"]);
            }
            SuggestionType::SingleRecommendation => {
                fzf_command.args(&["--print-query", "--expect", "tab,enter"]);
            }
            _ => {}
        }

        if let Some(p) = opts.preview {
            fzf_command.args(&["--preview", &p]);
        }

        if let Some(q) = opts.query {
            fzf_command.args(&["--query", &q]);
        }

        if let Some(f) = opts.filter {
            fzf_command.args(&["--filter", &f]);
        }

        if let Some(h) = opts.header {
            fzf_command.args(&["--header", &h]);
        }

        if let Some(p) = opts.prompt {
            fzf_command.args(&["--prompt", &p]);
        }

        if let Some(pw) = opts.preview_window {
            fzf_command.args(&["--preview-window", &pw]);
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

        let child = fzf_command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(x) => x,
            Err(_) => {
                eprintln!("navi was unable to call fzf.\nPlease make sure it's correctly installed\nRefer to https://github.com/junegunn/fzf for more info.");
                process::exit(33)
            }
        };

        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("Unable to acquire stdin of fzf"))?;
        let result_map = stdin_fn(stdin).context("Failed to pass data to fzf")?;

        let out = child.wait_with_output().context("Failed to wait for fzf")?;

        let text = match out.status.code() {
            Some(0) | Some(1) | Some(2) => {
                String::from_utf8(out.stdout).context("Invalid utf8 received from fzf")?
            }
            Some(130) => process::exit(130),
            _ => {
                let err = String::from_utf8(out.stderr)
                    .unwrap_or_else(|_| "<stderr contains invalid UTF-8>".to_owned());
                panic!("External command failed:\n {}", err)
            }
        };

        let out = get_column(
            parse_output_single(text, opts.suggestion_type)?,
            opts.column,
            opts.delimiter.as_deref(),
        );

        Ok((out, result_map))
    }
}
