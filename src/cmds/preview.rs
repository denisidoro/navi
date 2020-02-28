use ansi_term::Colour;
use std::error::Error;
use std::process;

use crate::option::Config;

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split('\t').skip(3);
    // println!("{:#?}", parts);
    let tags = parts.next().unwrap();
    let comment = parts.next().unwrap();
    let snippet = parts.next().unwrap();
    (tags, comment, snippet)
}

pub fn main(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Value for config: {:#?}", matches.subcommand().1.unwrap().value_of("line").unwrap());

    /*let (tags, comment, snippet) =
        extract_elements(matches.subcommand().1.unwrap().value_of("line").unwrap()); // ("foo", "bar", "baz"); // extract_elements(&args[2]);
    println!(
        "{comment} {tags} \n{snippet}",
        comment = Colour::Blue.paint(comment),
        tags = Colour::Red.paint(format!("[{}]", tags)),
        snippet = snippet
    );*/

    process::exit(0)
}
