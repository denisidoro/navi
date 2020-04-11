use super::{parse, Finder};
use crate::display;
use crate::structures::cheat::VariableMap;
use crate::structures::finder::{Opts, SuggestionType};
use anyhow::Context;
use anyhow::Error;
use std::process::{self, Command, Stdio};

#[derive(Debug)]
pub struct SkimFinder;

impl Finder for SkimFinder {
    fn call<F>(
        &self,
        finder_opts: Opts,
        stdin_fn: F,
    ) -> Result<(String, Option<VariableMap>), Error>
    where
        F: Fn(&mut process::ChildStdin) -> Result<Option<VariableMap>, Error>,
    {
        let mut command = Command::new("sk");
        let opts = finder_opts.clone();

        command.args(&[
            "--preview-window",
            "up:3",
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
            // TODO skim doesn't support this yet
            // this option does nothing
            command.arg("--select-1");
        }

        match opts.suggestion_type {
            SuggestionType::MultipleSelections => {
                command.arg("--multi");
            }
            SuggestionType::Disabled => {
                command.args(&["--print-query", /*"--no-select-1",*/ "--height", "1"]);
            }
            SuggestionType::SnippetSelection => {
                command.args(&["--expect", "ctrl-y,enter"]);
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

        let child = command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn();

        let mut child = match child {
            Ok(x) => x,
            Err(_) => {
                eprintln!("navi was unable to call skim.\nPlease make sure it's correctly installed\nRefer to https://github.com/junegunn/skim for more info.");
                process::exit(33)
            }
        };

        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("Unable to acquire stdin of skim"))?;
        let result_map = stdin_fn(stdin).context("Failed to pass data to skim")?;

        let out = child
            .wait_with_output()
            .context("Failed to wait for skim")?;

        let output = parse(out, finder_opts).context("Unable to get output")?;
        Ok((output, result_map))
    }
}
