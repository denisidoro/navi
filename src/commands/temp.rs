use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::{prelude::*, serializer};
use std::io::{self, Write};

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let _opts = FinderOpts::snippet_default();

    let fetcher = config.fetcher();
    let hash: u64 = 2087294461664323320;

    // let mut stdout = stdout();
    // let mut writer: Box<&mut dyn Write> = Box::new(&mut stdout);
    // let mut parser = Parser::new(&mut writer, false);

    let mut buf = vec![];
    let mut parser = Parser::new(&mut buf, false);
    parser.set_hash(hash);

    let _res = fetcher
        .fetch(&mut parser)
        .context("Failed to parse variables intended for finder")?;

    /*
    if let Some(variables) = res {
        Ok(Some(variables))
    } else {
        welcome::populate_cheatsheet(stdin)?;
        Ok(Some(VariableMap::new()))
    }
    */

    // Do writing here.

    let variables = parser.variables;
    let item_str = String::from_utf8(buf)?;
    let item = serializer::raycast_deser(&item_str)?;
    dbg!(&item);

    let x = variables.get_suggestion(&item.tags, "local_branch");
    dbg!(&x);

    Ok(())
}

pub fn main0() -> Result<()> {
    let config = &CONFIG;

    let fetcher = config.fetcher();

    let mut stdout = io::stdout();
    let mut writer: Box<&mut dyn Write> = Box::new(&mut stdout);
    let mut parser = Parser::new(&mut writer, false);

    let _res = fetcher
        .fetch(&mut parser)
        .context("Failed to parse variables intended for finder")?;

    Ok(())
}
