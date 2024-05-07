use crate::deser;
use crate::prelude::*;
use std::io::Write;
use std::process::{self, Output};
use std::process::{Command, Stdio};
pub mod structures;
use clap::ValueEnum;
pub use post::process;
use structures::Opts;
use structures::SuggestionType;

const MIN_FZF_VERSION_MAJOR: u32 = 0;
const MIN_FZF_VERSION_MINOR: u32 = 23;
const MIN_FZF_VERSION_PATCH: u32 = 1;

mod post;

#[derive(Debug, Clone, Copy, Deserialize, ValueEnum)]
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

fn parse(out: Output, opts: Opts) -> Result<String> {
    let text = match out.status.code() {
        Some(0) | Some(1) | Some(2) => {
            String::from_utf8(out.stdout).context("Invalid utf8 received from finder")?
        }
        Some(130) => process::exit(130),
        _ => {
            let err = String::from_utf8(out.stderr)
                .unwrap_or_else(|_| "<stderr contains invalid UTF-8>".to_owned());
            panic!("External command failed:\n {err}")
        }
    };

    let output = post::parse_output_single(text, opts.suggestion_type)?;
    post::process(output, opts.column, opts.delimiter.as_deref(), opts.map)
}

impl FinderChoice {
    fn check_fzf_version() -> Option<(u32, u32, u32)> {
        let output = Command::new("fzf").arg("--version").output().ok()?.stdout;
        let version_string = String::from_utf8(output).ok()?;
        let version_parts: Vec<_> = version_string.split('.').collect();
        if version_parts.len() == 3 {
            let major = version_parts[0].parse().ok()?;
            let minor = version_parts[1].parse().ok()?;
            let patch = version_parts[2].split_whitespace().next()?.parse().ok()?;
            Some((major, minor, patch))
        } else {
            None
        }
    }

    pub fn call<F, R>(&self, finder_opts: Opts, stdin_fn: F) -> Result<(String, R)>
    where
        F: Fn(&mut dyn Write) -> Result<R>,
    {
        let finder_str = match self {
            Self::Fzf => "fzf",
            Self::Skim => "sk",
        };

        if let Self::Fzf = self {
            if let Some((major, minor, patch)) = Self::check_fzf_version() {
                if major == MIN_FZF_VERSION_MAJOR
                    && minor < MIN_FZF_VERSION_MINOR
                    && patch < MIN_FZF_VERSION_PATCH
                {
                    eprintln!(
                        "Warning: Fzf version {major}.{minor} does not support the preview window layout used by navi.",
                    );
                    eprintln!(
                        "Consider updating Fzf to a version >= {MIN_FZF_VERSION_MAJOR}.{MIN_FZF_VERSION_MINOR}.{MIN_FZF_VERSION_PATCH} or use a compatible layout.",
                    );
                    process::exit(1);
                }
            }
        }

        let mut command = Command::new(finder_str);
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

        command.args([
            "--preview",
            "",
            "--preview-window",
            format!("up:{preview_height}:nohidden").as_str(),
            "--delimiter",
            deser::terminal::DELIMITER.to_string().as_str(),
            "--ansi",
            "--bind",
            format!("ctrl-j:down,ctrl-k:up{bindings}").as_str(),
            "--exact",
        ]);

        if !opts.show_all_columns {
            command.args(["--with-nth", "1,2,3"]);
        }

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
                    command.args(["--print-query", "--no-select-1"]);
                };
            }
            SuggestionType::SnippetSelection => {
                command.args(["--expect", "ctrl-y,ctrl-o,enter"]);
            }
            SuggestionType::SingleRecommendation => {
                command.args(["--print-query", "--expect", "tab,enter"]);
            }
            _ => {}
        }

        if let Some(p) = opts.preview {
            command.args(["--preview", &p]);
        }

        if let Some(q) = opts.query {
            command.args(["--query", &q]);
        }

        if let Some(f) = opts.filter {
            command.args(["--filter", &f]);
        }

        if let Some(d) = opts.delimiter {
            command.args(["--delimiter", &d]);
        }

        if let Some(h) = opts.header {
            command.args(["--header", &h]);
        }

        if let Some(p) = opts.prompt {
            command.args(["--prompt", &p]);
        }

        if let Some(pw) = opts.preview_window {
            command.args(["--preview-window", &pw]);
        }

        if opts.header_lines > 0 {
            command.args(["--header-lines", format!("{}", opts.header_lines).as_str()]);
        }

        if let Some(o) = opts.overrides {
            shellwords::split(&o)?
                .into_iter()
                .filter(|s| !s.is_empty())
                .for_each(|s| {
                    command.arg(s);
                });
        }

        command
            .env("SHELL", CONFIG.finder_shell())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        debug!(cmd = ?command);

        let child = command.spawn();

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

        let return_value = stdin_fn(&mut writer).context("Failed to pass data to finder")?;

        let out = child.wait_with_output().context("Failed to wait for finder")?;

        let output = parse(out, finder_opts).context("Unable to get output")?;
        Ok((output, return_value))
    }
}
