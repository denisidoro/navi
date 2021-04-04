use crate::common::terminal_width;
use crate::display;
use crate::env_vars;
use crate::finder;
use crate::structures::item::Item;
use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::iter;
use std::str::FromStr;
use termion::color;

// TODO: extract
pub fn parse_env_var<T: FromStr>(varname: &str) -> Option<T> {
    if let Ok(x) = env::var(varname) {
        x.parse::<T>().ok()
    } else {
        None
    }
}

lazy_static! {
    pub static ref TAG_COLOR: color::AnsiValue =
        color::AnsiValue(parse_env_var(env_vars::TAG_COLOR).unwrap_or(14));
    pub static ref COMMENT_COLOR: color::AnsiValue =
        color::AnsiValue(parse_env_var(env_vars::COMMENT_COLOR).unwrap_or(4));
    pub static ref SNIPPET_COLOR: color::AnsiValue =
        color::AnsiValue(parse_env_var(env_vars::SNIPPET_COLOR).unwrap_or(7));
    pub static ref TAG_WIDTH_PERCENTAGE: u16 = parse_env_var(env_vars::TAG_WIDTH).unwrap_or(20);
    pub static ref COMMENT_WIDTH_PERCENTAGE: u16 = parse_env_var(env_vars::COMMENT_WIDTH).unwrap_or(40);
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

pub fn wrapped_by_map(text: &str, map: Option<&str>) -> String {
    if map.is_none() {
        text.to_string()
    } else {
        format!("map({})", text)
    }
}

fn get_env_var(name: &str) -> String {
    if let Ok(v) = env::var(name) {
        v
    } else {
        panic!("{} not set", name)
    }
}

pub fn preview_var(selection: &str, query: &str, variable: &str) {
    let snippet = &get_env_var(env_vars::PREVIEW_INITIAL_SNIPPET);
    let tags = get_env_var(env_vars::PREVIEW_TAGS);
    let comment = get_env_var(env_vars::PREVIEW_COMMENT);
    let column = display::terminal::parse_env_var(env_vars::PREVIEW_COLUMN);
    let delimiter = env::var(env_vars::PREVIEW_DELIMITER).ok();
    let map = env::var(env_vars::PREVIEW_MAP).ok();

    let reset = color::Fg(color::Reset);
    let active_color = color::Fg(*TAG_COLOR);
    let inactive_color = color::Fg(*COMMENT_COLOR);

    let mut colored_snippet = String::from(snippet);
    let mut variables = String::from("");
    let mut visited_vars: HashSet<&str> = HashSet::new();

    let bracketed_current_variable = format!("<{}>", variable);

    let bracketed_variables: Vec<&str> = {
        if snippet.contains(&bracketed_current_variable) {
            display::VAR_REGEX
                .find_iter(snippet)
                .map(|m| m.as_str())
                .collect()
        } else {
            iter::once(&bracketed_current_variable)
                .map(|s| s.as_str())
                .collect()
        }
    };

    for bracketed_variable_name in bracketed_variables {
        let variable_name = &bracketed_variable_name[1..bracketed_variable_name.len() - 1];

        if visited_vars.contains(variable_name) {
            continue;
        } else {
            visited_vars.insert(variable_name);
        }

        let is_current = variable_name == variable;
        let variable_color = if is_current { active_color } else { inactive_color };
        let env_variable_name = variable_name.replace('-', "_");

        let value = if is_current {
            let v = selection.trim_matches('\'');
            if v.is_empty() { query.trim_matches('\'') } else { v }.to_string()
        } else if let Ok(v) = env::var(&env_variable_name) {
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

        colored_snippet = colored_snippet.replace(bracketed_variable_name, &replacement);

        variables = format!(
            "{variables}\n{color}{variable}{reset} = {value}",
            variables = variables,
            color = variable_color,
            variable = variable_name,
            reset = reset,
            value = wrapped_by_map(
                &finder::get_column(value, column, delimiter.as_deref()),
                map.as_deref()
            )
        );
    }

    println!(
        "{comment_color}{comment} {tag_color}{tags}{reset} \n{snippet}\n{variables}",
        comment = comment,
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
        display::terminal::Writer {
            tag_width,
            comment_width,
        }
    }
}

impl display::Writer for Writer {
    fn write(&mut self, item: Item) -> String {
        format!(
            "{tag_color}{tags_short}{delimiter}{comment_color}{comment_short}{delimiter}{snippet_color}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}{file_index}{delimiter}\n",
            tags_short = limit_str(item.tags, self.tag_width),
            comment_short = limit_str(item.comment, self.comment_width),
            snippet_short = display::fix_newlines(item.snippet),
            comment_color = color::Fg(*COMMENT_COLOR),
            tag_color = color::Fg(*TAG_COLOR),
            snippet_color = color::Fg(*SNIPPET_COLOR),
            tags = item.tags,
            comment = item.comment,
            delimiter = display::DELIMITER,
            snippet = &item.snippet,
            file_index = item.file_index,
        )
    }
}
