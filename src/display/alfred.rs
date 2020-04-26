use crate::display;
use crate::structures::item::Item;

pub struct Writer {
    is_first: bool,
}

fn escape_for_json(txt: &str) -> String {
    txt.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(display::NEWLINE_ESCAPE_CHAR, " ")
}

pub fn print_items_start(varname: Option<&str>) {
    print!("{{");

    if let Some(v) = varname {
        print!(r#""variables": {{"varname": "{varname}"}},"#, varname = v);
    }

    println!(r#""items": ["#);
}

pub fn print_items_end() {
    println!(r#"]}}"#);
}

impl display::Writer for Writer {
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
            r#"{prefix}{{"type":"file","title":"{comment}","match":"{comment} {tags} {snippet}","subtitle":"{tags} :: {snippet}","variables":{{"tags":"{tags}","comment":"{comment}","snippet":"{snippet}"}},"icon":{{"path":"icon.png"}}}}"#,
            prefix = prefix,
            tags = tags,
            comment = comment,
            snippet = snippet
        )
    }
}

impl Writer {
    pub fn new() -> Writer {
        Writer { is_first: true }
    }

    pub fn reset(&mut self) {
        self.is_first = true
    }

    pub fn write_suggestion(&mut self, snippet: &str, varname: &str, line: &str) {
        if line.len() < 3 {
            return;
        }

        let prefix = if self.is_first {
            self.is_first = false;
            ""
        } else {
            ","
        };

        println!(
            r#"{prefix}{{"title":"{value}","subtitle":"{snippet}","variables":{{"{varname}":"{value}"}},"icon":{{"path":"navi.png"}}}}"#,
            prefix = prefix,
            snippet = snippet,
            varname = varname,
            value = line
        );
    }
}
