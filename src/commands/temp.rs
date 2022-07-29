use crate::commands::core::get_fetcher;
use crate::common::shell::{self, ShellSpawnError};
use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::{deser, prelude::*};
use std::io::{self, Write};

pub fn main() -> Result<()> {
    let _config = &CONFIG;
    let _opts = FinderOpts::snippet_default();

    let fetcher = get_fetcher()?;
    let hash: u64 = 2087294461664323320;

    let mut buf = vec![];
    let mut parser = Parser::new(&mut buf, false);
    parser.set_hash(hash);

    let _res = fetcher
        .fetch(&mut parser)
        .context("Failed to parse variables intended for finder")?;

    let variables = parser.variables;
    let item_str = String::from_utf8(buf)?;
    let item = deser::raycast::read(&item_str)?;
    dbg!(&item);

    let x = variables.get_suggestion(&item.tags, "local_branch").expect("foo");
    dbg!(&x);

    let suggestion_command = x.0.clone();
    let child = shell::out()
        .stdout(Stdio::piped())
        .arg(&suggestion_command)
        .spawn()
        .map_err(|e| ShellSpawnError::new(suggestion_command, e))?;

    let text = String::from_utf8(
        child
            .wait_with_output()
            .context("Failed to wait and collect output from bash")?
            .stdout,
    )
    .context("Suggestions are invalid utf8")?;

    dbg!(&text);

    Ok(())
}

pub fn _main0() -> Result<()> {
    let _config = &CONFIG;

    let fetcher = get_fetcher()?;

    let mut stdout = io::stdout();
    let mut writer: Box<&mut dyn Write> = Box::new(&mut stdout);
    let mut parser = Parser::new(&mut writer, false);

    let _res = fetcher
        .fetch(&mut parser)
        .context("Failed to parse variables intended for finder")?;

    Ok(())
}
