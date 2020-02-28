use ansi_term::Colour;
use std::error::Error;
use std::process;

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split('\t').skip(3);
    // println!("{:#?}", parts);
    let tags = parts.next().unwrap();
    let comment = parts.next().unwrap();
    let snippet = parts.next().unwrap();
    (tags, comment, snippet)
}

pub fn main(line: String) -> Result<(), Box<dyn Error>> {
    let (tags, comment, snippet) = extract_elements(&line[..]);
    println!(
        "{comment} {tags} \n{snippet}",
        comment = Colour::Blue.paint(comment),
        tags = Colour::Red.paint(format!("[{}]", tags)),
        snippet = snippet
    );

    process::exit(0)
}
