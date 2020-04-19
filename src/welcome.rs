use crate::display::{self, Writer};
use crate::structures::item::Item;
use std::io::Write;

fn add_msg(tags: &str, comment: &str, snippet: &str, writer: &mut Box<dyn Writer>, stdin: &mut std::process::ChildStdin) {
        let item = Item { tags: &tags, comment: &comment, snippet: &snippet }; 
    stdin
        .write_all(writer.write(item).as_bytes())
        .expect("Could not write to fzf's stdin");
}

pub fn cheatsheet(writer: &mut Box<dyn Writer>, stdin: &mut std::process::ChildStdin) {
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
