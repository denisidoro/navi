use crate::finder::structures::Opts;
use crate::finder::FinderChoice;
use crate::prelude::*;

pub fn finder_yes_no_question(finder: &FinderChoice, opts: Opts) -> anyhow::Result<bool> {
    let (response, _) = finder
        .call(opts, |stdin| {
            stdin
                .write_all(b"Yes\nNo")
                .context("Unable to writer alternatives")?;
            Ok(())
        })
        .context("Unable to get response")?;

    Ok(response.to_lowercase().starts_with('y'))
}
