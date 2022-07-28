use crate::finder::structures::Opts as FinderOpts;
use crate::parser::Parser;
use crate::prelude::*;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let _opts = FinderOpts::snippet_default();

    let mut fetcher = config.fetcher();
    // let hash: u64 = 1531163706200719240;

    // let mut stdout = stdout();
    // let mut writer: Box<&mut dyn Write> = Box::new(&mut stdout);
    let mut buf = vec![];
    // let mut parser = Parser::new(&mut writer, false);
    let mut parser = Parser::new(&mut buf, false);

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

    let string = String::from_utf8(buf)?;
    dbg!(&string);

    Ok(())
}
