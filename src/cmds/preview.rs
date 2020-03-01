use std::error::Error;
use std::process;

use crate::display;
use termion::color;

fn extract_elements(argstr: &str) -> (&str, &str, &str) {
    let mut parts = argstr.split('\t').skip(3);
    let tags = parts.next().unwrap();
    let comment = parts.next().unwrap();
    let snippet = parts.next().unwrap();
    (tags, comment, snippet)
}

pub fn main(line: String) -> Result<(), Box<dyn Error>> {
    let (tags, comment, snippet) = extract_elements(&line[..]);

    println!(
        "{comment_color}{comment} {tag_color}{tags} \n{snippet_color}{snippet}",
        comment = format!("# {}", comment),
        tags = format!("[{}]", tags),
        snippet = snippet,
        comment_color = color::Fg(display::COMMENT_COLOR),
        tag_color = color::Fg(display::TAG_COLOR),
        snippet_color = color::Fg(display::SNIPPET_COLOR),
    );

    process::exit(0)
}
