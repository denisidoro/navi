use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::fzf;
use crate::parse;
use crate::option::Config;

pub fn main(_config: Config) -> Result<(), Box<dyn Error>> {
    let (output, variables) = fzf::call(|stdin| parse::read_all(stdin));

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let mut parts = raw_output.split('\t');
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

            let (output, _) = fzf::call(|stdin| {
                stdin.write_all(suggestions.as_bytes()).unwrap();
                HashMap::new()
            });

            let value = String::from_utf8(output.stdout).unwrap();
            full_snippet = full_snippet.replace(bracketed_varname, &value[..value.len() - 1]);
        }

        Command::new("bash")
            .arg("-c")
            .arg(&full_snippet[..])
            .spawn()?;

        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        panic!("External command failed:\n {}", err)
    }
}
