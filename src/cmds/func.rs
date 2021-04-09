use crate::handler;
use crate::structures::config;
use crate::url;
use anyhow::Error;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Func {
    UrlOpen,
    Welcome,
    Shell,
}

pub fn main(func: &Func, args: Vec<String>) -> Result<(), Error> {
    match func {
        Func::UrlOpen => url::open(args),
        Func::Welcome => handler::handle_config(config::config_from_iter(
            "navi --path /tmp/navi/irrelevant".split(' ').collect(),
        )),
        Func::Shell => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text)?;

            let replacements = vec![("|", "ඛ"), ("||", "ග"), ("&&", "ඝ")];

            let parts =
                shellwords::split(&text).map_err(|_| anyhow!("Given options are missing a closing quote"))?;

            for p in parts {
                for (pattern, escaped) in replacements.clone() {
                    if p.contains(pattern) && p != pattern {
                        let replacement = p.replace(pattern, escaped);
                        text = text.replace(&p, &replacement);
                    }
                }
            }

            let mut extracted = text.clone();
            for (pattern, _) in replacements.clone() {
                let mut new_parts = text.rsplit(pattern);
                if let Some(extracted_attempt) = new_parts.next() {
                    if extracted_attempt.len() <= extracted.len() {
                        extracted = extracted_attempt.to_string();
                    }
                }
            }

            for (pattern, escaped) in replacements.clone() {
                text = text.replace(&escaped, &pattern);
                extracted = extracted.replace(&escaped, &pattern);
            }

            // println!("text: {}", text);
            // println!("extracted: {}", extracted);

            println!("{}", extracted.trim_start());

            Ok(())
        }
    }
}
