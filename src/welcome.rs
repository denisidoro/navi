use crate::actor;
use crate::config::CONFIG;
use crate::extractor;
use crate::finder::structures::Opts as FinderOpts;
use crate::finder::Finder;
use crate::structures::cheat::VariableMap;
use crate::structures::item::Item;
use crate::writer;
use anyhow::Context;
use anyhow::Result;
use std::io::Write;

pub fn main() -> Result<()> {
    let config = &CONFIG;
    let opts = FinderOpts::snippet_default();

    let (raw_selection, variables, files) = config
        .finder()
        .call(opts, |stdin, _| {
            populate_cheatsheet(stdin);
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

fn add_msg(tags: &str, comment: &str, snippet: &str, stdin: &mut std::process::ChildStdin) {
    let item = Item {
        tags: tags.to_string(),
        comment: comment.to_string(),
        snippet: snippet.to_string(),
        file_index: 0,
    };
    stdin
        .write_all(writer::write(&item).as_bytes())
        .expect("Could not write to fzf's stdin");
}

pub fn populate_cheatsheet(stdin: &mut std::process::ChildStdin) {
    let items: Vec<(&str, &str, &str)> = vec![
        (
            "cheatsheets",
            "Download default cheatsheets",
            "navi repo add denisidoro/cheats",
        ),
        ("cheatsheets", "Browse for cheatsheet repos", "navi repo browse"),
        (
            "cheatsheet",
            "Edit main local cheatsheets",
            r#"f="$(navi info cheats-path)/main.cheat"; [ -f "$f" ] || echo -e "% first cheat\n\n# print something\necho hello world\n" > "$f"; $EDITOR "$f""#,
        ),
        (
            "widget",
            "Initialize shell widget",
            r#"eval "$(navi widget $SHELL)""#,
        ),
        (
            "config",
            "Edit config file",
            r#"f="$(navi info config-path)"; [ -f "$f" ] || navi info config-example > "$f"; $EDITOR "$f""#,
        ),
        ("more info", "Read --help message", "navi --help"),
    ];

    for (tags, comment, snippet) in items {
        add_msg(tags, comment, snippet, stdin);
    }
}
