mod fzf;
mod skim;

use crate::structures::cheat::VariableMap;
use crate::structures::finder::Opts;
use crate::structures::finder::SuggestionType;
use anyhow::Context;
use anyhow::Error;
pub use fzf::FzfFinder;
pub use skim::SkimFinder;
use std::process::{self, Output};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum FinderChoice {
    Fzf,
    Skim,
}

pub trait Finder {
    fn call<F>(&self, opts: Opts, stdin_fn: F) -> Result<(String, Option<VariableMap>), Error>
    where
        F: Fn(&mut process::ChildStdin) -> Result<Option<VariableMap>, Error>;
}

impl Finder for FinderChoice {
    fn call<F>(&self, opts: Opts, stdin_fn: F) -> Result<(String, Option<VariableMap>), Error>
    where
        F: Fn(&mut process::ChildStdin) -> Result<Option<VariableMap>, Error>,
    {
        match self {
            Self::Fzf => FzfFinder.call(opts, stdin_fn),
            Self::Skim => SkimFinder.call(opts, stdin_fn),
        }
    }
}

fn apply_map(text: String, map_fn: Option<String>) -> String {
    if let Some(m) = map_fn {
        let output = Command::new("bash")
            .arg("-c")
            .arg(m.as_str())
            .arg(text.as_str())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to execute map function");

        String::from_utf8(output.stdout).expect("Invalid utf8 output for map function")
    } else {
        text
    }
}

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

fn parse_output_single(mut text: String, suggestion_type: SuggestionType) -> Result<String, Error> {
    Ok(match suggestion_type {
        SuggestionType::SingleSelection => text
            .lines()
            .next()
            .context("Not sufficient data for single selection")?
            .to_string(),
        SuggestionType::MultipleSelections
        | SuggestionType::Disabled
        | SuggestionType::SnippetSelection => {
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
                (Some(one), Some(termination), None)
                    if *termination == "enter" || termination.is_empty() =>
                {
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
    let output = get_column(output, opts.column, opts.delimiter.as_deref());
    let output = apply_map(output, opts.map);
    Ok(output)
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
