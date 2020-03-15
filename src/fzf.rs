use crate::cheat;
use crate::display;
use crate::filesystem;

use crate::cheat::SuggestionType;
use crate::cheat::SuggestionType::SingleSelection;
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
    pub header: Option<String>,
    pub suggestion_type: SuggestionType,
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
            header: None,
            prompt: None,
            suggestion_type: SingleSelection,
        }
    }
}

pub fn call<F>(opts: Opts, stdin_fn: F) -> (String, Option<HashMap<String, cheat::Suggestion>>)
where
    F: Fn(&mut process::ChildStdin) -> Option<HashMap<String, cheat::Suggestion>>,
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

    if let Some(h) = opts.header {
        fzf_command.args(&["--header", &h]);
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

    let text = match out.status.code() {
        Some(0) | Some(1) => String::from_utf8(out.stdout).unwrap(),
        Some(130) => process::exit(130),
        _ => {
            let err = String::from_utf8(out.stderr)
                .unwrap_or_else(|_| "<stderr contains invalid UTF-8>".to_owned());
            panic!("External command failed:\n {}", err)
        }
    };
    (parse_output_single(text, opts.suggestion_type), result)
}

fn parse_output_single(mut text: String, suggestion_type: SuggestionType) -> String {
    match suggestion_type {
        SuggestionType::SingleSelection => text.lines().next().unwrap().to_string(),
        SuggestionType::MultipleSelections
        | SuggestionType::Disabled
        | SuggestionType::SnippetSelection => {
            text.truncate(text.len() - 1);
            text
        }
        SuggestionType::SingleRecommendation => {
            let lines: Vec<&str> = text.lines().collect();

            match (lines.get(0), lines.get(1), lines.get(2)) {
                (Some(one), Some(termination), Some(two)) if *termination == "enter" => {
                    if two.is_empty() {
                        (*one).to_string()
                    } else {
                        (*two).to_string()
                    }
                }
                (Some(one), Some(termination), None) if *termination == "enter" => {
                    (*one).to_string()
                }
                (Some(one), Some(termination), _) if *termination == "tab" => (*one).to_string(),
                _ => "".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_output1() {
        let text = "palo\n".to_string();
        let output = parse_output_single(text, SuggestionType::SingleSelection);
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_output2() {
        let text = "\nenter\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation);
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_recommendation_output_1() {
        let text = "\nenter\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation);
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_recommendation_output_2() {
        let text = "p\nenter\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation);
        assert_eq!(output, "palo");
    }

    #[test]
    fn test_parse_recommendation_output_3() {
        let text = "peter\nenter\n".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation);
        assert_eq!(output, "peter");
    }

    #[test]
    fn test_parse_output3() {
        let text = "p\ntab\npalo".to_string();
        let output = parse_output_single(text, SuggestionType::SingleRecommendation);
        assert_eq!(output, "p");
    }

    #[test]
    fn test_parse_snippet_request() {
        let text = "enter\nssh                     ⠀login to a server and forward to ssh key (d…  ⠀ssh -A <user>@<server>  ⠀ssh  ⠀login to a server and forward to ssh key (dangerous but usefull for bastion hosts)  ⠀ssh -A <user>@<server>  ⠀\n".to_string();
        let output = parse_output_single(text, SuggestionType::SnippetSelection);
        assert_eq!(output,     "enter\nssh                     ⠀login to a server and forward to ssh key (d…  ⠀ssh -A <user>@<server>  ⠀ssh  ⠀login to a server and forward to ssh key (dangerous but usefull for bastion hosts)  ⠀ssh -A <user>@<server>  ⠀");
    }
}
