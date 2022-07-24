use crate::actor;
use crate::extractor;
use crate::finder::structures::Opts as FinderOpts;
use crate::finder::Finder;
use crate::parser;
use crate::prelude::*;
use crate::structures::cheat::VariableMap;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let (raw_selection, variables, files) = config
        .finder()
        .call(opts, |stdin, _| {
            populate_cheatsheet(stdin)?;
            Ok(Some(VariableMap::new()))
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match());

    if extractions.is_err() {
        return main();
    }

    actor::act(extractions, files, variables)?;
    Ok(())
}

pub fn populate_cheatsheet(stdin: &mut std::process::ChildStdin) -> Result<()> {
    let cheatsheet = include_str!("../docs/navi.cheat");

    parser::read_lines(
        cheatsheet.split('\n').into_iter().map(|s| Ok(s.to_string())),
        "welcome",
        0,
        &mut VariableMap::new(),
        &mut Default::default(),
        stdin,
        None,
        None,
    )?;

    Ok(())
}
