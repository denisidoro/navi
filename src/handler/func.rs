use crate::cheat_variable;
use crate::prelude::*;
use crate::shell;
use crate::url;
use crate::welcome;

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
