use crate::cheat_variable;
use crate::prelude::*;
use crate::shell;
use crate::url;
use crate::welcome;
use clap::Args;
use clap::Parser;

const FUNC_POSSIBLE_VALUES: &[&str] = &["url::open", "welcome", "widget::last_command", "map::expand"];

impl FromStr for Func {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "url::open" => Ok(Func::UrlOpen),
            "welcome" => Ok(Func::Welcome),
            "widget::last_command" => Ok(Func::WidgetLastCommand),
            "map::expand" => Ok(Func::MapExpand),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub enum Func {
    UrlOpen,
    Welcome,
    WidgetLastCommand,
    MapExpand,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    /// Function name (example: "url::open")
    #[clap(possible_values = FUNC_POSSIBLE_VALUES, ignore_case = true)]
    pub func: Func,
    /// List of arguments (example: "https://google.com")
    pub args: Vec<String>,
}

pub fn main(input: &Input) -> Result<()> {
    let func = &input.func;
    let args = input.args;

    match func {
        Func::UrlOpen => url::open(args),
        Func::Welcome => welcome::main(),
        Func::WidgetLastCommand => shell::widget_last_command(),
        Func::MapExpand => cheat_variable::map_expand(),
    }
}
