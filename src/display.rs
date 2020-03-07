use crate::terminal;
use termion::color;

static COMMENT_COLOR: color::LightCyan = color::LightCyan;
static TAG_COLOR: color::LightGreen = color::LightGreen;
static SNIPPET_COLOR: color::White = color::White;

lazy_static! {
    pub static ref WIDTHS: (usize, usize) = get_widths();
}

fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width = width * 10 / 100;
    let comment_width = width * 50 / 100;
    (usize::from(tag_width), usize::from(comment_width))
}

pub fn variable_prompt(varname: &str) -> String {
    format!("{}: ", varname)
}

pub fn preview(comment: &str, tags: &str, snippet: &str) {
    println!(
        "{comment_color}{comment} {tag_color}{tags} \n{snippet_color}{snippet}",
        comment = format!("# {}", comment),
        tags = format!("[{}]", tags),
        snippet = snippet,
        comment_color = color::Fg(COMMENT_COLOR),
        tag_color = color::Fg(TAG_COLOR),
        snippet_color = color::Fg(SNIPPET_COLOR),
    );
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", &text[..length - 1])
    } else {
        format!("{:width$}", text, width = length)
    }
}

pub fn format_line(
    tags: &str,
    comment: &str,
    full_snippet: &str,
    tag_width: usize,
    comment_width: usize,
) -> String {
    format!(
       "{tag_color}{tags_short}\t{comment_color}{comment_short}\t{snippet_color}{snippet_short}\t{tags}\t{comment}\t{snippet}\t\n",
       tags_short = limit_str(tags, tag_width),
       comment_short = limit_str(comment, comment_width),
       snippet_short = full_snippet,
       comment_color = color::Fg(COMMENT_COLOR),
       tag_color = color::Fg(TAG_COLOR),
       snippet_color = color::Fg(SNIPPET_COLOR),
       tags = tags,
       comment = comment,
       snippet = &full_snippet)
}
