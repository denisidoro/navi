use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::cheat;
use crate::fzf;
use crate::option::Config;

pub enum Variant {
    Core,
    Filter(String),
    Query(String),
}

pub fn main(variant: Variant, config: Config) -> Result<(), Box<dyn Error>> {
    let fzf_opts = match variant {
        Variant::Core => Default::default(),
        Variant::Filter(f) => fzf::Opts {
            filter: Some(f.to_string()),
            ..Default::default()
        },
        Variant::Query(q) => fzf::Opts {
            query: Some(q.to_string()),
            ..Default::default()
        },
    };

    let (output, variables) = fzf::call(fzf_opts, |stdin| cheat::read_all(&config, stdin));

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let mut parts = raw_output.split('\n').next().unwrap().split('\t');
        parts.next();
        parts.next();
        parts.next();
        let tags = parts.next().unwrap();
        parts.next();
        let snippet = parts.next().unwrap();
        let mut full_snippet = String::from(snippet);

        let re = Regex::new(r"<(.*?)>").unwrap();
        for cap in re.captures_iter(snippet) {
            let bracketed_varname = &cap[0];
            let varname = &bracketed_varname[1..bracketed_varname.len() - 1];
            let k = format!("{};{}", tags, varname);

            let suggestions = match variables.get(&k[..]) {
                Some(c) => {
                    let child = Command::new("bash")
                        .stdout(Stdio::piped())
                        .arg("-c")
                        .arg(c)
                        .spawn()
                        .unwrap();

                    String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap()
                }
                None => String::from("TODO\n"),
            };

            let (output, _) = fzf::call(Default::default(), |stdin| {
                stdin.write_all(suggestions.as_bytes()).unwrap();
                HashMap::new() // TODO
            });

            let value = String::from_utf8(output.stdout).unwrap();
            full_snippet = full_snippet.replace(bracketed_varname, &value[..value.len() - 1]);
        }

        if config.print {
            println!("{}", full_snippet);
        } else {
            Command::new("bash")
                .arg("-c")
                .arg(&full_snippet[..])
                .spawn()?;
        }

        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        panic!("External command failed:\n {}", err)
    }
}
