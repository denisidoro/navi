use crate::env_var;
use crate::finder;
use crate::structures::item::Item;
use crate::terminal;
use crate::terminal::style::{style, Color};
use crate::writer;
use std::cmp::max;
use std::collections::HashSet;
use std::iter;

fn parse_ansi(varname: &str, default: Color) -> Color {
    let value: Option<String> = env_var::parse(varname);
    if let Some(v) = value {
        if let Some(a) = terminal::parse_ansi(&v) {
            return a;
        }
    }
    default
}

lazy_static! {
    pub static ref TAG_COLOR: Color = parse_ansi(env_var::TAG_COLOR, Color::Cyan);
    pub static ref COMMENT_COLOR: Color = parse_ansi(env_var::COMMENT_COLOR, Color::Blue);
    pub static ref SNIPPET_COLOR: Color = parse_ansi(env_var::SNIPPET_COLOR, Color::White);
    pub static ref TAG_WIDTH_PERCENTAGE: u16 = env_var::parse(env_var::TAG_WIDTH).unwrap_or(27);
    pub static ref COMMENT_WIDTH_PERCENTAGE: u16 = env_var::parse(env_var::COMMENT_WIDTH).unwrap_or(43);
}

pub fn preview(comment: &str, tags: &str, snippet: &str) {
    println!(
        "{comment} {tags} \n{snippet}",
        comment = style(comment).with(*COMMENT_COLOR),
        tags = style(format!("[{}]", tags)).with(*TAG_COLOR),
        snippet = style(writer::fix_newlines(snippet)).with(*SNIPPET_COLOR),
    );
}
pub fn preview_var(selection: &str, query: &str, variable: &str) {
    let snippet = env_var::must_get(env_var::PREVIEW_INITIAL_SNIPPET);
    let tags = env_var::must_get(env_var::PREVIEW_TAGS);
    let comment = env_var::must_get(env_var::PREVIEW_COMMENT);
    let column = env_var::parse(env_var::PREVIEW_COLUMN);
    let delimiter = env_var::get(env_var::PREVIEW_DELIMITER).ok();
    let map = env_var::get(env_var::PREVIEW_MAP).ok();

    let active_color = *TAG_COLOR;
    let inactive_color = *COMMENT_COLOR;

    let mut colored_snippet = String::from(&snippet);
    let mut visited_vars: HashSet<&str> = HashSet::new();

    let mut variables = String::from("");

    println!(
        "{comment} {tags}",
        comment = style(comment).with(*COMMENT_COLOR),
        tags = style(format!("[{}]", tags)).with(*TAG_COLOR),
    );

    let bracketed_current_variable = format!("<{}>", variable);

    let bracketed_variables: Vec<&str> = {
        if snippet.contains(&bracketed_current_variable) {
            writer::VAR_REGEX
                .find_iter(&snippet)
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
        let env_variable_name = env_var::escape(variable_name);

        let value = if is_current {
            let v = selection.trim_matches('\'');
            if v.is_empty() { query.trim_matches('\'') } else { v }.to_string()
        } else if let Ok(v) = env_var::get(&env_variable_name) {
            v
        } else {
            "".to_string()
        };

        let replacement = format!(
            "{variable}",
            variable = style(bracketed_variable_name).with(variable_color),
        );

        colored_snippet = colored_snippet.replace(bracketed_variable_name, &replacement);

        variables = format!(
            "{variables}\n{variable} = {value}",
            variables = variables,
            variable = style(variable_name).with(variable_color),
            value = finder::process(value, column, delimiter.as_deref(), map.clone())
                .expect("Unable to process value"),
        );
    }

    println!("{snippet}", snippet = writer::fix_newlines(&colored_snippet));
    println!("{variables}", variables = variables);
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", text.chars().take(length - 1).collect::<String>())
    } else {
        format!("{:width$}", text, width = length)
    }
}

fn get_widths() -> (usize, usize) {
    let width = terminal::width();
    let tag_width = max(20, width * *TAG_WIDTH_PERCENTAGE / 100);
    let comment_width = max(60, width * *COMMENT_WIDTH_PERCENTAGE / 100);
    (usize::from(tag_width), usize::from(comment_width))
}

pub struct Writer {
    tag_width: usize,
    comment_width: usize,
}

impl Writer {
    pub fn new() -> Writer {
        let (tag_width, comment_width) = get_widths();
        writer::terminal::Writer {
            tag_width,
            comment_width,
        }
    }
}

impl writer::Writer for Writer {
    fn write(&mut self, item: Item) -> String {
        format!(
            "{tags_short}{delimiter}{comment_short}{delimiter}{snippet_short}{delimiter}{tags}{delimiter}{comment}{delimiter}{snippet}{delimiter}{file_index}{delimiter}\n",
            tags_short = style(limit_str(item.tags, self.tag_width)).with(*TAG_COLOR),
            comment_short = style(limit_str(item.comment, self.comment_width)).with(*COMMENT_COLOR),
            snippet_short = style(writer::fix_newlines(item.snippet)).with(*SNIPPET_COLOR),
            tags = item.tags,
            comment = item.comment,
            delimiter = writer::DELIMITER,
            snippet = &item.snippet,
            file_index = item.file_index,
        )
    }
}
