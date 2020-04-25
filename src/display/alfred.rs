use crate::structures::item::Item;
use crate::display::{self, Writer};

pub struct AlfredWriter {
    is_first: bool,
}

pub fn new_writer() -> AlfredWriter {
AlfredWriter { is_first: false }
}

fn escape_for_json(txt: &str) -> String {
    txt.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(display::NEWLINE_ESCAPE_CHAR, " ")
}

impl Writer for AlfredWriter {
    fn write(&mut self, item: Item) -> String {
        let prefix = if self.is_first {
            self.is_first = false;
            ""
        } else {
            ","
        };

        let tags = escape_for_json(item.tags);
        let comment = escape_for_json(item.comment);
        let snippet = escape_for_json(item.snippet);

        format!(
            r#"{prefix}{{"type":"file","title":"{comment}","match":"{comment} {tags} {snippet}","subtitle":"{tags} :: {snippet}","variables":{{"tags":"{tags}","comment":"{comment}","snippet":"{snippet}"}},"icon":{{"path":"navi.png"}}}}"#,
            prefix = prefix,
            tags = tags,
            comment = comment,
            snippet = snippet
        )
    }
}
