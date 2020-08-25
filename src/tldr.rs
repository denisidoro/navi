use regex::Regex;
use crate::structures::config::Config;
use crate::structures::error::filesystem::InvalidPath;
use crate::structures::error::filesystem::UnreadableDir;
use anyhow::{Context, Error};
use core::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use crate::display::Writer;
use crate::structures::cheat::VariableMap;
use crate::welcome;
use crate::parser;
use std::collections::HashSet;

lazy_static! {
    pub static ref VAR_TLDR_REGEX: Regex =
        Regex::new(r"\{\{(.*?)\}\}").expect("Invalid regex");
    pub static ref NON_VAR_CHARS_REGEX: Regex =
        Regex::new(r"[^\da-zA-Z_]").expect("Invalid regex");
}

static MARKDOWN: &'static str = r#"# tar

> Archiving utility.
> Often combined with a compression method, such as gzip or bzip.
> More information: <https://www.gnu.org/software/tar>.

- Create an archive from files:

`tar cf {{target.tar}} {{file1}} {{file2}} {{file3}}`

- Create a gzipped archive:

`tar czf {{target.tar.gz}} {{file1}} {{file2}} {{file3}}`

- Create a gzipped archive from a directory using relative paths:

`tar czf {{target.tar.gz}} -C {{path/to/directory}} .`

- Extract a (compressed) archive into the current directory:

`tar xf {{source.tar[.gz|.bz2|.xz]}}`

- Extract a (compressed) archive into the target directory:

`tar xf {{source.tar[.gz|.bz2|.xz]}} -C {{directory}}`

- Create a compressed archive, using archive suffix to determine the compression program:

`tar caf {{target.tar.xz}} {{file1}} {{file1}} {{file2}} {{file3}}`

- List the contents of a tar file:

`tar tvf {{source.tar}}`

- Extract files matching a pattern:

`tar xf {{source.tar}} --wildcards {{"*.html"}}`

- Extract a specific file without preserving the folder structure:

`tar xf {{source.tar}} {{source.tar/path/to/extract}} --strip-components={{depth_to_strip}}`"#;

fn convert_tldr_vars(line: &str) -> String {
    let caps = VAR_TLDR_REGEX.find_iter(&line);
    let mut new_line: String = line.to_string();
    for cap in caps {
        let braced_var = cap.as_str();
        let var = &braced_var[2..braced_var.len()-2];
        let new_var = NON_VAR_CHARS_REGEX.replace_all(var, "_");
        let bracketed_var = format!("<{}>", new_var);
        new_line = new_line.replace(braced_var, &bracketed_var);
    }
    new_line
}

fn convert_tldr(line: &str) -> Result<String, Error> {
    let new_line = if line.starts_with('-') {
        format!("{}{}", "# ", &line[2..line.len()-1])
    } else if line.starts_with('`') {
        String::from(convert_tldr_vars(&line[1..line.len()-1]))
    } else if line.starts_with('%') {
        line.to_string()
    } else {
        "".to_string()
    };
    Ok(new_line)
}

fn markdown_lines() -> impl Iterator<Item=Result<String, Error>> {
    let prefix = r#"% markdown, test
    "#.lines().map(|line| convert_tldr(line));
    let lines = MARKDOWN.lines().map(|line| convert_tldr(line.trim()));
    prefix.chain(lines)
}

pub fn read_all(
    config: &Config,
    stdin: &mut std::process::ChildStdin,
    writer: &mut dyn Writer,
) -> Result<VariableMap, Error> {
    let mut variables = VariableMap::new();
    let mut visited_lines = HashSet::new();
    parser::read_lines(markdown_lines(), "markdown", &mut variables, &mut visited_lines, writer, stdin)?;
    Ok(variables) 
}
