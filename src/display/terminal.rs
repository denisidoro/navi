use crate::common::terminal_width;
use crate::display;
use crate::structures::item::Item;
use std::cmp::max;
use std::collections::HashSet;
use std::env;
use termion::color;

fn parse_env_var_u8(varname: &str) -> Option<u8> {
    if let Ok(x) = env::var(varname) {
        x.parse::<u8>().ok()
    } else {
        None
    }
}

fn parse_env_var_u16(varname: &str) -> Option<u16> {
    if let Ok(x) = env::var(varname) {
        x.parse::<u16>().ok()
    } else {
        None
    }
}

lazy_static! {
    pub static ref TAG_COLOR: color::AnsiValue = color::AnsiValue(parse_env_var_u8("NAVI_TAG_COLOR").unwrap_or(14));
    pub static ref COMMENT_COLOR: color::AnsiValue = color::AnsiValue(parse_env_var_u8("NAVI_COMMENT_COLOR").unwrap_or(4));
    pub static ref SNIPPET_COLOR: color::AnsiValue = color::AnsiValue(parse_env_var_u8("NAVI_SNIPPET_COLOR").unwrap_or(7));
    pub static ref TAG_WIDTH_PERCENTAGE: u16 = parse_env_var_u16("NAVI_TAG_WIDTH").unwrap_or(20);
    pub static ref COMMENT_WIDTH_PERCENTAGE: u16 = parse_env_var_u16("NAVI_COMMENT_WIDTH").unwrap_or(40);
}

pub fn variable_prompt(varname: &str) -> String {
    format!("{}: ", varname)
}

pub fn preview(comment: &str, tags: &str, snippet: &str) {
    println!(
        "{comment_color}{comment} {tag_color}{tags} \n{snippet_color}{snippet}",
        comment = comment.to_string(),
        tags = format!("[{}]", tags),
        snippet = display::fix_newlines(snippet),
        comment_color = color::Fg(*COMMENT_COLOR),
        tag_color = color::Fg(*TAG_COLOR),
        snippet_color = color::Fg(*SNIPPET_COLOR),
    );
}

pub fn preview2(snippet: &str, tags: &str, comment: &str, selection: &str, query: &str, variable: &str) {
    let reset = color::Fg(color::Reset);
    let active_color = color::Fg(*TAG_COLOR);
    let inactive_color = color::Fg(*SNIPPET_COLOR);
    let mut colored_snippet = String::from(snippet);
    let mut variables = String::from("");
    let mut visited_vars: HashSet<&str> = HashSet::new();
    for bracketed_variable_name in display::VAR_REGEX.find_iter(snippet).map(|m| m.as_str()) {
        let variable_name = &bracketed_variable_name[1..bracketed_variable_name.len() - 1];
        if visited_vars.contains(variable_name) {
            continue;
        } else {
            visited_vars.insert(variable_name);
        }
        let is_current = variable_name == variable;
        let variable_color = if is_current { active_color } else { inactive_color };
        let value = if is_current {
            let v = selection.trim_matches('\'');
            if v.is_empty() { query.trim_matches('\'') } else { v }.to_string()
        } else if let Ok(v) = env::var(&variable_name) {
            v
        } else {
            "".to_string()
        };
        let replacement = format!(
            "{color}{variable}{reset}",
            color = variable_color,
            variable = bracketed_variable_name,
            reset = reset
        );
        colored_snippet = colored_snippet.replacen(bracketed_variable_name, &replacement, 999);
        variables = format!(
            "{variables}\n{color}{variable}{reset} = {value}",
            variables = variables,
            color = variable_color,
            variable = variable_name,
            reset = reset,
            value = value
        );
    }
    println!(
        "{comment_color}{comment} {tag_color}{tags}{reset} \n{snippet}\n{variables}",
        comment = comment.to_string(),
        tags = format!("[{}]", tags),
        snippet = display::fix_newlines(&colored_snippet),
        comment_color = color::Fg(*COMMENT_COLOR),
        tag_color = color::Fg(*TAG_COLOR),
        variables = variables,
        reset = reset
    );
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", text.chars().take(length - 1).collect::<String>())
    } else {
        format!("{:width$}", text, width = length)
    }
}

fn get_widths() -> (usize, usize) {
    let width = terminal_width::get();
    let tag_width = max(4, width * *TAG_WIDTH_PERCENTAGE / 100);
    let comment_width = max(4, width * *COMMENT_WIDTH_PERCENTAGE / 100);
    (usize::from(tag_width), usize::from(comment_width))
}

pub struct Writer {
    tag_width: usize,
    comment_width: usize,
}

impl Writer {
    pub fn new() -> Writer {
        let (tag_width, comment_width) = get_widths();
        display::terminal::Writer { tag_width, comment_width }
    }
}

impl display::Writer for Writer {
    fn write(&mut self, item: Item) -> String {
        format!(
       "{tag_color}{tags_short}{delimiter}{comment_color}{comment_short}{delimiter}{snippet_color}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}\n",
       tags_short = limit_str(item.tags, self.tag_width),
       comment_short = limit_str(item.comment, self.comment_width),
       snippet_short = display::fix_newlines(item.snippet),
       comment_color = color::Fg(*COMMENT_COLOR),
       tag_color = color::Fg(*TAG_COLOR),
       snippet_color = color::Fg(*SNIPPET_COLOR),
       tags = item.tags,
       comment = item.comment,
       delimiter = display::DELIMITER,
       snippet = &item.snippet)
    }
}
