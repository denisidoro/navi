use crate::display;
use crate::structures::cheat::VariableMap;
use crate::structures::finder::Opts;
use crate::structures::finder::SuggestionType;
use anyhow::Context;
use anyhow::Error;
use std::process::{self, Output};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum FinderChoice {
    Fzf,
    Skim,
}

pub trait Finder {
    fn call<F>(
        &self,
        opts: Opts,
        files: &mut Vec<String>,
        stdin_fn: F,
    ) -> Result<(String, Option<VariableMap>), Error>
    where
        F: Fn(&mut process::ChildStdin, &mut Vec<String>) -> Result<Option<VariableMap>, Error>;
}

// TODO: extract
fn apply_map(text: String, map_fn: Option<String>) -> Result<String, Error> {
    if let Some(m) = map_fn {
        let cmd = format!(
            r#"
_navi_map_fn() {{
  {}
}}
                
read -r -d '' _navi_input <<'NAVIEOF'
{}
NAVIEOF

echo "$_navi_input" | _navi_map_fn"#,
            m, text
        );

        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd.as_str())
            .stderr(Stdio::inherit())
            .output()
            .context("Failed to execute map function")?;

        String::from_utf8(output.stdout).context("Invalid utf8 output for map function")
    } else {
        Ok(text)
    }
}

// TODO: extract
fn get_column(text: String, column: Option<u8>, delimiter: Option<&str>) -> String {
    if let Some(c) = column {
        let mut result = String::from("");
        let re = regex::Regex::new(delimiter.unwrap_or(r"\s\s+")).expect("Invalid regex");
        for line in text.split('\n') {
            if (&line).is_empty() {
                continue;
            }
            let mut parts = re.split(line).skip((c - 1) as usize);
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(parts.next().unwrap_or(""));
        }
        result
    } else {
        text
    }
}

// TODO: extract
pub fn process(
    text: String,
    column: Option<u8>,
    delimiter: Option<&str>,
    map_fn: Option<String>,
) -> Result<String, Error> {
    apply_map(get_column(text, column, delimiter), map_fn)
}

fn parse_output_single(mut text: String, suggestion_type: SuggestionType) -> Result<String, Error> {
    Ok(match suggestion_type {
        SuggestionType::SingleSelection => text
            .lines()
            .next()
            .context("Not sufficient data for single selection")?
            .to_string(),
        SuggestionType::MultipleSelections | SuggestionType::Disabled | SuggestionType::SnippetSelection => {
            let len = text.len();
            if len > 1 {
                text.truncate(len - 1);
            }
            text
        }
        SuggestionType::SingleRecommendation => {
            let lines: Vec<&str> = text.lines().collect();

            match (lines.get(0), lines.get(1), lines.get(2)) {
                (Some(one), Some(termination), Some(two))
                    if *termination == "enter" || termination.is_empty() =>
                {
                    if two.is_empty() {
                        (*one).to_string()
                    } else {
                        (*two).to_string()
                    }
                }
                (Some(one), Some(termination), None) if *termination == "enter" || termination.is_empty() => {
                    (*one).to_string()
                }
                (Some(one), Some(termination), _) if *termination == "tab" => (*one).to_string(),
                _ => "".to_string(),
            }
        }
    })
}

fn parse(out: Output, opts: Opts) -> Result<String, Error> {
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

    let output = parse_output_single(text, opts.suggestion_type)?;
    process(output, opts.column, opts.delimiter.as_deref(), opts.map)
}

impl Finder for FinderChoice {
    fn call<F>(
        &self,
        finder_opts: Opts,
        files: &mut Vec<String>,
        stdin_fn: F,
    ) -> Result<(String, Option<VariableMap>), Error>
    where
        F: Fn(&mut process::ChildStdin, &mut Vec<String>) -> Result<Option<VariableMap>, Error>,
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
            display::DELIMITER.to_string().as_str(),
            "--ansi",
            "--bind",
            format!("ctrl-j:down,ctrl-k:up{}", bindings).as_str(),
            "--exact",
        ]);

        if let Self::Fzf = self {
            command.arg("--select-1");
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
        let result_map = stdin_fn(stdin, files).context("Failed to pass data to finder")?;

        let out = child.wait_with_output().context("Failed to wait for finder")?;

        let output = parse(out, finder_opts).context("Unable to get output")?;
        Ok((output, result_map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output1() {
        let text = "palo\n".to_string();
        let output = parse_output_single(text, SuggestionType::SingleSelection).unwrap();
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_output2() {
        let text = "\nenter\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation).unwrap();
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_recommendation_output_1() {
        let text = "\nenter\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation).unwrap();
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_recommendation_output_2() {
        let text = "p\nenter\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation).unwrap();
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_recommendation_output_3() {
        let text = "peter\nenter\n".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation).unwrap();
        assert_eq!(output, "peter");
    }

    #[test]
    fn test_parse_output3() {
        let text = "p\ntab\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation).unwrap();
        assert_eq!(output, "p");
    }

    #[test]
    fn test_parse_snippet_request() {
        let text = "enter\nssh                     ⠀login to a server and forward to ssh key (d…  ⠀ssh -A <user>@<server>  ⠀ssh  ⠀login to a server and forward to ssh key (dangerous but usefull for bastion hosts)  ⠀ssh -A <user>@<server>  ⠀\n".to_string();
        let output = parse_output_single(text, SuggestionType::SnippetSelection).unwrap();
        assert_eq!(output,     "enter\nssh                     ⠀login to a server and forward to ssh key (d…  ⠀ssh -A <user>@<server>  ⠀ssh  ⠀login to a server and forward to ssh key (dangerous but usefull for bastion hosts)  ⠀ssh -A <user>@<server>  ⠀");
    }
}
