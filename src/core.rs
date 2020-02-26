use clap::ArgMatches;
use regex::Regex;
use std::error::Error;
use std::io::Write;
use std::process::Command;

use crate::fzf;
use crate::parse;

pub fn main(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let output = fzf::call(|stdin| {
        parse::read_all(stdin);
    });

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let snippet = raw_output.split('\t').nth(5).unwrap();
        let mut full_snippet = String::from(snippet);

        let re = Regex::new(r"<(.*?)>").unwrap();
        for cap in re.captures_iter(snippet) {
            println!("{}", &cap[0]);
            let bracketed_varname = &cap[0];
            let varname = &bracketed_varname[1..bracketed_varname.len() - 1];

            let output = fzf::call(|stdin| {
                stdin.write_all(b"foo\n").unwrap();
                stdin.write_all(b"bar\n").unwrap();
                stdin.write_all(b"baz\n").unwrap();
                stdin
                    .write_all(format!("{}\n", varname).as_bytes())
                    .unwrap();
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
