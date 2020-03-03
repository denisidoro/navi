use crate::cheat;
use crate::cmds;
use crate::display;
use crate::fzf;
use crate::option::Config;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::process;
use std::process::{Command, Stdio};

pub enum Variant {
    Core,
    Filter(String),
    Query(String),
}

fn gen_core_fzf_opts(variant: Variant, config: &Config) -> fzf::Opts {
    let mut opts = fzf::Opts {
        preview: !config.no_preview,
        autoselect: !config.no_autoselect,
        overrides: config.fzf_overrides.as_ref(),
        copyable: true,
        ..Default::default()
    };

    match variant {
        Variant::Core => (),
        Variant::Filter(f) => opts.filter = Some(f),
        Variant::Query(q) => opts.query = Some(q),
    }

    opts
}

fn extract_from_selections(raw_output: &str) -> (&str, &str, &str) {
    let mut lines = raw_output.split('\n');
    let key = lines.next().unwrap();
    let mut parts = lines.next().unwrap().split('\t');
    parts.next();
    parts.next();
    parts.next();
    let tags = parts.next().unwrap();
    parts.next();
    let snippet = parts.next().unwrap();
    (key, tags, snippet)
}

fn prompt_with_suggestions(config: &Config, suggestion: &cheat::Value) -> String {
    let child = Command::new("bash")
        .stdout(Stdio::piped())
        .arg("-c")
        .arg(&suggestion.0)
        .spawn()
        .unwrap();

    let suggestions = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();

    let mut opts = fzf::Opts {
        preview: false,
        autoselect: !config.no_autoselect,
        ..Default::default()
    };

    if let Some(o) = &suggestion.1 {
        opts.multi = o.multi;
        opts.header_lines = o.header_lines;
        opts.nth = o.column;
    };

    let (output, _) = fzf::call(opts, |stdin| {
        stdin.write_all(suggestions.as_bytes()).unwrap();
        None
    });

    String::from_utf8(output.stdout).unwrap()
}

fn prompt_without_suggestions(varname: &str) -> String {
    let opts = fzf::Opts {
        preview: false,
        autoselect: false,
        suggestions: false,
        prompt: Some(display::variable_prompt(varname)),
        ..Default::default()
    };

    let (output, _) = fzf::call(opts, |_stdin| None);

    String::from_utf8(output.stdout).unwrap()
}

fn gen_replacement(value: &str) -> String {
    if value.contains(" ") {
        format!("\"{}\"", &value[..value.len() - 1])
    } else {
        format!("{}", &value[..value.len() - 1])
    }
}

fn replace_variables_from_snippet(
    snippet: &str,
    tags: &str,
    variables: HashMap<String, cheat::Value>,
    config: &Config,
) -> String {
    let mut interpolated_snippet = String::from(snippet);

    let re = Regex::new(r"<(\w[\w\d\-_]*)>").unwrap();
    for cap in re.captures_iter(snippet) {
        let bracketed_varname = &cap[0];
        let varname = &bracketed_varname[1..bracketed_varname.len() - 1];
        let k = format!("{};{}", tags, varname);

        let value = match variables.get(&k[..]) {
            Some(suggestion) => prompt_with_suggestions(&config, suggestion),
            None => prompt_without_suggestions(varname),
        };

        interpolated_snippet =
            interpolated_snippet.replace(bracketed_varname, gen_replacement(&value[..]).as_str());
    }

    interpolated_snippet
}

pub fn main(variant: Variant, config: Config) -> Result<(), Box<dyn Error>> {
    let (output, variables) = fzf::call(gen_core_fzf_opts(variant, &config), |stdin| {
        Some(cheat::read_all(&config, stdin))
    });

    match output.status.code() {
        Some(0) => {
            let raw_output = String::from_utf8(output.stdout)?;
            let (key, tags, snippet) = extract_from_selections(&raw_output[..]);
            let interpolated_snippet =
                replace_variables_from_snippet(snippet, tags, variables.unwrap(), &config);

            if key == "ctrl-y" {
                cmds::aux::abort("copying snippets to the clipboard", 201)?
            } else if config.print {
                println!("{}", interpolated_snippet);
            } else {
                Command::new("bash")
                    .arg("-c")
                    .arg(&interpolated_snippet[..])
                    .spawn()?;
            }

            Ok(())
        }
        Some(130) => process::exit(130),
        _ => {
            let err = String::from_utf8(output.stderr)?;
            panic!("External command failed:\n {}", err)
        }
    }
}
