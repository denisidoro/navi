use crate::config::CONFIG;

use crate::cheat_variable;
use crate::finder::Finder;
use crate::handler::core;
use crate::shell::{self, ShellSpawnError};
use crate::structures::cheat::VariableMap;
use crate::url;
use crate::welcome;
use anyhow::Context;
use anyhow::Result;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Func {
    UrlOpen,
    Welcome,
    WidgetLastCommand,
    MapExpand,
}

pub fn main(func: &Func, args: Vec<String>) -> Result<()> {
    match func {
        Func::UrlOpen => url::open(args),
        Func::Welcome => welcome::main(),
        Func::WidgetLastCommand => shell::widget_last_command(),
        Func::MapExpand => cheat_variable::map_expand(),
    }
}
