use clap::ArgMatches;
use regex::Regex;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use std::collections::HashMap;

use crate::fzf;
use crate::parse;

pub fn main(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let (output, variables) = fzf::call(|stdin| {
        parse::read_all(stdin)
    });

    // println!("{}, {:#?}", variables.len(), variables);

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let mut parts = raw_output.split('\t');
        parts.next();
        parts.next();
        parts.next();
        let tags = parts.next().unwrap();
        let comment = parts.next().unwrap();
        let snippet = parts.next().unwrap();
        let mut full_snippet = String::from(snippet);
        
        let re = Regex::new(r"<(.*?)>").unwrap();
        for cap in re.captures_iter(snippet) {
            println!("tags: {}; comment: {}; snippet: {}; cap: {}", tags, comment, snippet, &cap[0]);
            let bracketed_varname = &cap[0];
            let varname = &bracketed_varname[1..bracketed_varname.len() - 1];
            let k = format!("{};{}", tags, varname);

            let suggestions = match variables.get(&k[..]) {
                Some(c) => {
                    println!("c: {}", c);

                   let child = Command::new("bash")
                   .stdout(Stdio::piped())
                    .arg("-c")
                    .arg(c)
                    .spawn()
                    .unwrap();
                    
                    String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap()
                },
                None => String::from("TODO\n"),
            };

            let (output, _) = fzf::call(|stdin| {
                stdin.write_all(b"before\n").unwrap();
                stdin.write_all(suggestions.as_bytes()).unwrap();
                stdin.write_all(b"after\n").unwrap();
                HashMap::new()
            });

            let value = String::from_utf8(output.stdout).unwrap();
            full_snippet = full_snippet.replace(bracketed_varname, &value[..value.len() - 1]);

            println!("suggestions: {}; snippet: {}", suggestions, full_snippet);
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
