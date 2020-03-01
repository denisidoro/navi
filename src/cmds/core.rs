use crate::cheat;
use crate::cmds;
use crate::fzf;
use crate::option::Config;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};
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

fn extract(raw_output: &str) -> (&str, &str, &str) {
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

pub fn main(variant: Variant, config: Config) -> Result<(), Box<dyn Error>> {
    let (output, variables) = fzf::call(gen_core_fzf_opts(variant, &config), |stdin| {
        cheat::read_all(&config, stdin)
    });

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let (key, tags, snippet) = extract(&raw_output[..]);
        let mut full_snippet = String::from(snippet);

        let re = Regex::new(r"<(\w[\w\d\-_]*)>").unwrap();
        for cap in re.captures_iter(snippet) {
            let bracketed_varname = &cap[0];
            let varname = &bracketed_varname[1..bracketed_varname.len() - 1];
            let k = format!("{};{}", tags, varname);

            let value = match variables.get(&k[..]) {
                Some(suggestion) => {
                    let child = Command::new("bash")
                        .stdout(Stdio::piped())
                        .arg("-c")
                        .arg(&suggestion.0)
                        .spawn()
                        .unwrap();

                    let suggestions =
                        String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();

                    let mut opts = fzf::Opts {
                        preview: false,
                        autoselect: !config.no_autoselect,
                        ..Default::default()
                    };

                    if let Some(o) = &suggestion.1 {
                        opts.multi = o.multi;
                    }

                    let (sub_output, _) = fzf::call(opts, |stdin| {
                        stdin.write_all(suggestions.as_bytes()).unwrap();
                        HashMap::new() // TODO
                    });

                    String::from_utf8(sub_output.stdout).unwrap()
                }
                None => {
                    let reader = stdin();
                    let mut out = stdout();
                    let mut buffer: String = String::new();

                    out.write(b"foo: ").unwrap();
                    reader.read_line(&mut buffer).unwrap();
                    buffer.to_string()
                }
            };

            full_snippet = full_snippet.replace(bracketed_varname, &value[..value.len() - 1]);
        }

        if key == "ctrl-y" {
            cmds::aux::abort("copying snippets to the clipboard", 2)?
        } else if config.print {
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
