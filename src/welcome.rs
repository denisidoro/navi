use crate::display;
use std::io::Write;

fn add_msg(tag: &str, comment: &str, snippet: &str, stdin: &mut std::process::ChildStdin) {
    stdin
        .write_all(display::format_line(tag, comment, snippet, 10, 60).as_bytes())
        .unwrap();
}

pub fn cheatsheet(stdin: &mut std::process::ChildStdin) {
    add_msg(
        "cheatsheets",
        "Download default cheatsheets",
        "navi repo add denisidoro/cheats",
        stdin,
    );
    add_msg("more info", "Read --help message", "navi --help", stdin);
}
