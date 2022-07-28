use crate::actor;
use crate::extractor;
use crate::finder::structures::Opts as FinderOpts;

use crate::parser::Parser;
use crate::prelude::*;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let (raw_selection, variables) = config
        .finder()
        .call(opts, |writer| {
            let mut parser = Parser::new(writer, true);
            populate_cheatsheet(&mut parser)?;
            Ok(Some(parser.variables))
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match());

    if extractions.is_err() {
        return main();
    }

    actor::act(extractions, vec![], variables)?;
    Ok(())
}

pub fn populate_cheatsheet(parser: &mut Parser) -> Result<()> {
    let cheatsheet = include_str!("../docs/navi.cheat");

    parser.read_lines(
        cheatsheet.split('\n').into_iter().map(|s| Ok(s.to_string())),
        "welcome",
        None,
    )?;

    Ok(())
}
