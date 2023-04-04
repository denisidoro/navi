mod map;
mod widget;

use super::core;
use super::temp;
use crate::common::url;
use crate::prelude::*;
use clap::Args;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Func {
    #[value(name = "url::open")]
    UrlOpen,
    #[value(name = "welcome")]
    Welcome,
    #[value(name = "widget::last_command")]
    WidgetLastCommand,
    #[value(name = "map::expand")]
    MapExpand,
    #[value(name = "temp")]
    Temp,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    /// Function name (example: "url::open")
    #[arg(ignore_case = true)]
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
