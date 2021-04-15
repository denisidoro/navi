use crate::structures::item::Item;
use crate::writer::Writer;
use std::io::Write;

fn add_msg(
    tags: &str,
    comment: &str,
    snippet: &str,
    writer: &mut dyn Writer,
    stdin: &mut std::process::ChildStdin,
) {
    let item = Item {
        tags: tags.to_string(),
        comment: comment.to_string(),
        snippet: snippet.to_string(),
        file_index: 0,
    };
    stdin
        .write_all(writer.write(&item).as_bytes())
        .expect("Could not write to fzf's stdin");
}

pub fn populate_cheatsheet(writer: &mut dyn Writer, stdin: &mut std::process::ChildStdin) {
    add_msg(
        "cheatsheets",
        "Download default cheatsheets",
        "navi repo add denisidoro/cheats",
        writer,
        stdin,
    );
    add_msg(
        "cheatsheets",
        "Browse for cheatsheet repos",
        "navi repo browse",
        writer,
        stdin,
    );
    add_msg("more info", "Read --help message", "navi --help", writer, stdin);
}
