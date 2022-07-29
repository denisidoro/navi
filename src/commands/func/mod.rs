mod map;
mod widget;

use super::core;
use super::temp;
use crate::common::url;
use crate::prelude::*;
use clap::Args;
use clap::Parser;

const POSSIBLE_VALUES: &[&str] = &[
    "url::open",
    "welcome",
    "widget::last_command",
    "map::expand",
    "temp",
];

impl FromStr for Func {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "url::open" => Ok(Func::UrlOpen),
            "welcome" => Ok(Func::Welcome),
            "widget::last_command" => Ok(Func::WidgetLastCommand),
            "map::expand" => Ok(Func::MapExpand),
            "temp" => Ok(Func::Temp),
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
    Temp,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    /// Function name (example: "url::open")
    #[clap(possible_values = POSSIBLE_VALUES, ignore_case = true)]
    pub func: Func,
    /// List of arguments (example: "https://google.com")
    pub args: Vec<String>,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        let func = &self.func;
        let args = self.args.clone(); // TODO

        match func {
            Func::UrlOpen => url::open(args),
            Func::Welcome => core::main(),
            Func::WidgetLastCommand => widget::last_command(),
            Func::MapExpand => map::expand(),
            Func::Temp => temp::main(),
        }
    }
}

#[test]
fn test_possible_values() {
    for v in POSSIBLE_VALUES {
        assert!(Func::from_str(v).is_ok())
    }
}
